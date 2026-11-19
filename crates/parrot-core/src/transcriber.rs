//! The loaded speech model and the recognition streams it serves.

use std::path::Path;
use std::sync::Arc;

use sherpa_onnx::{OnlineRecognizer, OnlineRecognizerConfig, OnlineStream};

use crate::error::{Error, Result};
use crate::language::Language;
use crate::models::ModelFiles;
use crate::transcript::Transcript;

/// Silence fed before the first audio so the encoder has left context for the
/// very first word.
const LEADING_PADDING_SECONDS: f32 = 0.3;
/// Silence appended when a dictation ends so the streaming encoder has enough
/// right context to emit the last words.
const TAIL_PADDING_SECONDS: f32 = 0.8;

/// Pause after speech that closes a segment.
const SEGMENT_PAUSE_SECONDS: f32 = 1.2;
/// Silence before any speech after which an empty segment is discarded.
const LEADING_SILENCE_SECONDS: f32 = 2.4;
/// Longest segment before the recognizer is forced to start a new one.
const MAX_SEGMENT_SECONDS: f32 = 20.0;

/// A loaded speech model, ready to transcribe.
///
/// Loading takes a few seconds and several hundred megabytes of memory, so a
/// single instance is shared by every dictation.
pub struct Transcriber {
    recognizer: OnlineRecognizer,
}

impl std::fmt::Debug for Transcriber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transcriber").finish_non_exhaustive()
    }
}

impl Transcriber {
    pub fn load(files: &ModelFiles) -> Result<Self> {
        let mut config = OnlineRecognizerConfig::default();
        config.model_config.transducer.encoder = Some(utf8_path(&files.encoder)?);
        config.model_config.transducer.decoder = Some(utf8_path(&files.decoder)?);
        config.model_config.transducer.joiner = Some(utf8_path(&files.joiner)?);
        config.model_config.tokens = Some(utf8_path(&files.tokens)?);
        config.model_config.num_threads = inference_threads();
        config.decoding_method = Some("greedy_search".to_owned());
        config.enable_endpoint = true;
        config.rule1_min_trailing_silence = LEADING_SILENCE_SECONDS;
        config.rule2_min_trailing_silence = SEGMENT_PAUSE_SECONDS;
        config.rule3_min_utterance_length = MAX_SEGMENT_SECONDS;

        let recognizer = OnlineRecognizer::create(&config)
            .ok_or_else(|| Error::ModelLoad(files.encoder.clone()))?;
        Ok(Self { recognizer })
    }

    /// Opens a new recognition stream for speech in `language`, recorded at
    /// `sample_rate`. The recognizer resamples to the rate the model expects.
    pub fn start_recognition(
        self: &Arc<Self>,
        language: Language,
        sample_rate: u32,
    ) -> Recognition {
        let stream = self.recognizer.create_stream();
        stream.set_option("language", language.code());
        let recognition = Recognition {
            transcriber: Arc::clone(self),
            stream,
            transcript: Transcript::default(),
            sample_rate: i32::try_from(sample_rate)
                .expect("audio sample rates are far below i32::MAX"),
        };
        recognition.feed_silence(LEADING_PADDING_SECONDS);
        recognition
    }
}

/// One ongoing transcription: audio goes in, text comes out incrementally.
pub struct Recognition {
    transcriber: Arc<Transcriber>,
    stream: OnlineStream,
    transcript: Transcript,
    /// The stream rejects a change of sample rate, so it is fixed at creation.
    sample_rate: i32,
}

impl std::fmt::Debug for Recognition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Recognition")
            .field("transcript", &self.transcript)
            .finish_non_exhaustive()
    }
}

impl Recognition {
    /// Feeds mono samples in `-1.0..=1.0`, recorded at the sample rate given
    /// when the recognition started, and decodes whatever is ready. Returns
    /// whether the transcript changed.
    pub fn accept(&mut self, samples: &[f32]) -> bool {
        self.stream.accept_waveform(self.sample_rate, samples);
        self.decode_ready()
    }

    /// The transcript so far.
    pub fn text(&self) -> String {
        self.transcript.text()
    }

    /// Flushes the audio still buffered in the model and returns the final
    /// transcript.
    pub fn finish(mut self) -> String {
        self.feed_silence(TAIL_PADDING_SECONDS);
        self.stream.input_finished();
        self.decode_ready();
        self.transcript.commit_partial();
        self.transcript.text()
    }

    fn feed_silence(&self, seconds: f32) {
        let len = (seconds * self.sample_rate as f32) as usize;
        self.stream
            .accept_waveform(self.sample_rate, &vec![0.0; len]);
    }

    fn decode_ready(&mut self) -> bool {
        let recognizer = &self.transcriber.recognizer;
        while recognizer.is_ready(&self.stream) {
            recognizer.decode(&self.stream);
        }
        let partial = recognizer
            .get_result(&self.stream)
            .map(|result| result.text)
            .unwrap_or_default();
        let mut changed = self.transcript.set_partial(&partial);

        if recognizer.is_endpoint(&self.stream) {
            changed |= self.transcript.commit_partial();
            recognizer.reset(&self.stream);
        }
        changed
    }
}

/// Threads used by ONNX Runtime. Half the cores, capped at four: beyond that
/// a single streaming decode gains little and the rest of the desktop suffers.
fn inference_threads() -> i32 {
    std::thread::available_parallelism().map_or(2, |cores| (cores.get() / 2).clamp(1, 4) as i32)
}

fn utf8_path(path: &Path) -> Result<String> {
    path.to_str()
        .map(str::to_owned)
        .ok_or_else(|| Error::NonUtf8Path(path.to_owned()))
}
