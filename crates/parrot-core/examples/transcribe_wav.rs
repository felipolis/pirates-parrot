//! Transcribes a WAV file with an installed model, feeding it in 100 ms
//! chunks exactly as live microphone audio would arrive, and reports how fast
//! the model runs compared to real time.
//!
//! ```text
//! cargo run --release -p parrot-core --example transcribe_wav -- <model-dir> <audio.wav> [language]
//! ```
//!
//! `language` is a locale code such as `pt-BR` and defaults to `auto`.

use std::env;
use std::path::Path;
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Instant;

use parrot_core::{Language, ModelFiles, Transcriber};
use sherpa_onnx::Wave;

const CHUNK_SECONDS: f32 = 0.1;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let (model_dir, wav_path, language) = match args.as_slice() {
        [model_dir, wav_path] => (model_dir, wav_path, Some(Language::Auto)),
        [model_dir, wav_path, code] => (model_dir, wav_path, Language::from_code(code)),
        _ => {
            eprintln!("usage: transcribe_wav <model-dir> <audio.wav> [language]");
            return ExitCode::FAILURE;
        }
    };
    let Some(language) = language else {
        eprintln!("unknown language code; use one of the codes listed in language.rs");
        return ExitCode::FAILURE;
    };
    let Some(wave) = Wave::read(wav_path) else {
        eprintln!("could not read {wav_path}");
        return ExitCode::FAILURE;
    };

    let loading = Instant::now();
    let transcriber = match Transcriber::load(&ModelFiles::in_dir(Path::new(model_dir))) {
        Ok(transcriber) => Arc::new(transcriber),
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };
    println!("model loaded in {:.1}s", loading.elapsed().as_secs_f32());

    let sample_rate = u32::try_from(wave.sample_rate()).expect("WAV sample rates are positive");
    let chunk_len = (sample_rate as f32 * CHUNK_SECONDS) as usize;
    let transcribing = Instant::now();
    let mut recognition = transcriber.start_recognition(language, sample_rate);
    for chunk in wave.samples().chunks(chunk_len) {
        if recognition.accept(chunk) {
            println!("  … {}", recognition.text());
        }
    }
    let text = recognition.finish();
    let elapsed = transcribing.elapsed().as_secs_f32();
    let duration = wave.samples().len() as f32 / sample_rate as f32;

    println!("\n{text}\n");
    println!(
        "{duration:.1}s of audio transcribed in {elapsed:.2}s (real-time factor {:.2})",
        elapsed / duration
    );
    ExitCode::SUCCESS
}
