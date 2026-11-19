//! Core engine of Pirate's Parrot.
//!
//! This crate knows nothing about windows, tray icons or hotkeys. It owns the
//! pieces that turn microphone audio into text:
//!
//! - [`ModelStore`] downloads, verifies and installs speech models.
//! - [`Transcriber`] loads a model and opens [`Recognition`] streams.
//! - [`Dictation`] wires the microphone to a recognition stream on background
//!   threads and reports progress through [`DictationEvent`]s.

mod audio;
mod dictation;
mod error;
mod language;
mod models;
mod transcriber;
mod transcript;

pub use audio::{InputDevice, list_input_devices};
pub use dictation::{Dictation, DictationEvent, DictationOptions};
pub use error::{Error, Result};
pub use language::Language;
pub use models::{InstallProgress, ModelFiles, ModelId, ModelSpec, ModelStore};
pub use transcriber::{Recognition, Transcriber};
