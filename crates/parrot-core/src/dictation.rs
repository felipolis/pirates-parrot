//! A dictation: the microphone wired to a recognition stream on a background
//! thread, reporting the live transcript and the voice level as it goes.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::audio::{AudioCapture, CaptureMessage};
use crate::error::{Error, Result};
use crate::language::Language;
use crate::transcriber::{Recognition, Transcriber};

/// Something that happened during a dictation.
#[derive(Debug, Clone, PartialEq)]
pub enum DictationEvent {
    /// The transcript changed. Carries the whole text so far.
    Transcript(String),
    /// Microphone loudness in `0.0..=1.0`, sent a few dozen times per second.
    Level(f32),
    /// The microphone stopped delivering audio. The dictation should be
    /// finished to keep what was transcribed so far.
    DeviceLost(String),
}

/// How a dictation should listen.
#[derive(Debug, Clone, Default)]
pub struct DictationOptions {
    pub language: Language,
    /// Identifier from [`crate::list_input_devices`], or `None` for the
    /// system default microphone.
    pub input_device: Option<String>,
}

/// A live dictation: the microphone is recording and audio is transcribed
/// on a background thread as it arrives.
///
/// End it with [`Dictation::finish`] to get the text, or [`Dictation::cancel`]
/// to throw it away. Dropping it cancels.
#[derive(Debug)]
pub struct Dictation {
    capture: Option<AudioCapture>,
    worker: Option<JoinHandle<String>>,
    cancelled: Arc<AtomicBool>,
}

impl Dictation {
    /// Starts recording and transcribing. `on_event` is called from a
    /// background thread.
    pub fn start(
        transcriber: &Arc<Transcriber>,
        options: &DictationOptions,
        on_event: impl FnMut(DictationEvent) + Send + 'static,
    ) -> Result<Self> {
        let (audio_tx, audio_rx) = mpsc::channel();
        // Audio recorded before the worker starts waits in the channel.
        let capture = AudioCapture::start(options.input_device.as_deref(), audio_tx)?;
        let recognition = transcriber.start_recognition(options.language, capture.sample_rate());
        let cancelled = Arc::new(AtomicBool::new(false));

        let worker = thread::Builder::new().name("dictation".to_owned()).spawn({
            let cancelled = Arc::clone(&cancelled);
            move || transcribe(recognition, &audio_rx, &cancelled, on_event)
        })?;

        Ok(Self {
            capture: Some(capture),
            worker: Some(worker),
            cancelled,
        })
    }

    /// Stops the microphone and waits for the final transcript.
    pub fn finish(mut self) -> Result<String> {
        self.capture.take();
        self.worker
            .take()
            .expect("the worker is only taken when the dictation ends")
            .join()
            .map_err(|_| Error::WorkerPanicked)
    }

    /// Stops the microphone and discards the transcript.
    pub fn cancel(self) {
        // Dropping performs the cancellation.
    }
}

impl Drop for Dictation {
    fn drop(&mut self) {
        let Some(worker) = self.worker.take() else {
            return;
        };
        self.cancelled.store(true, Ordering::Relaxed);
        self.capture.take();
        // A cancelled dictation has no result to report; a crash is still logged.
        if worker.join().is_err() {
            log::error!("the dictation worker panicked");
        }
    }
}

/// Worker loop: feeds microphone audio to the recognizer until the capture
/// stops, then returns the final transcript.
fn transcribe(
    mut recognition: Recognition,
    audio: &Receiver<CaptureMessage>,
    cancelled: &AtomicBool,
    mut on_event: impl FnMut(DictationEvent),
) -> String {
    let mut meter = LevelMeter::default();

    for message in audio {
        if cancelled.load(Ordering::Relaxed) {
            return String::new();
        }
        match message {
            CaptureMessage::Audio(samples) => {
                if let Some(level) = meter.measure(&samples) {
                    on_event(DictationEvent::Level(level));
                }
                if recognition.accept(&samples) {
                    on_event(DictationEvent::Transcript(recognition.text()));
                }
            }
            CaptureMessage::Failed(reason) => {
                on_event(DictationEvent::DeviceLost(reason));
                break;
            }
        }
    }

    if cancelled.load(Ordering::Relaxed) {
        return String::new();
    }
    recognition.finish()
}

/// Turns raw audio into a smooth loudness value at a steady rate.
#[derive(Debug)]
struct LevelMeter {
    peak_since_report: f32,
    last_report: Instant,
}

impl LevelMeter {
    const INTERVAL: Duration = Duration::from_millis(40);
    /// Quietest level shown, in dBFS. Anything below reads as silence.
    const FLOOR_DB: f32 = -55.0;

    /// Records a chunk and returns the loudest level since the previous
    /// report once per interval.
    fn measure(&mut self, samples: &[f32]) -> Option<f32> {
        self.peak_since_report = self
            .peak_since_report
            .max(loudness(samples, Self::FLOOR_DB));
        if self.last_report.elapsed() < Self::INTERVAL {
            return None;
        }
        self.last_report = Instant::now();
        Some(std::mem::take(&mut self.peak_since_report))
    }
}

impl Default for LevelMeter {
    fn default() -> Self {
        Self {
            peak_since_report: 0.0,
            last_report: Instant::now(),
        }
    }
}

/// RMS loudness mapped from `floor_db..=0` dBFS onto `0.0..=1.0`.
fn loudness(samples: &[f32], floor_db: f32) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let mean_square = samples.iter().map(|s| s * s).sum::<f32>() / samples.len() as f32;
    let db = 10.0 * mean_square.max(f32::MIN_POSITIVE).log10();
    ((db - floor_db) / -floor_db).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silence_is_zero() {
        assert_eq!(loudness(&[0.0; 160], -55.0), 0.0);
        assert_eq!(loudness(&[], -55.0), 0.0);
    }

    #[test]
    fn full_scale_is_one() {
        assert!((loudness(&[1.0, -1.0, 1.0, -1.0], -55.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn loudness_grows_with_amplitude() {
        let quiet = loudness(&[0.01; 160], -55.0);
        let loud = loudness(&[0.3; 160], -55.0);
        assert!(0.0 < quiet && quiet < loud && loud < 1.0);
    }
}
