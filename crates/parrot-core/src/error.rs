//! Errors of the speech engine.

use std::path::PathBuf;

/// Result type used throughout `parrot-core`.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Everything that can go wrong while capturing audio, transcribing it or
/// managing speech models.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no microphone is available")]
    NoInputDevice,

    #[error("the selected microphone is not connected")]
    InputDeviceNotFound,

    #[error("microphone error: {0}")]
    Audio(#[from] cpal::Error),

    #[error("the microphone uses an unsupported sample format ({0})")]
    UnsupportedSampleFormat(String),

    #[error("could not load the speech model from {}", .0.display())]
    ModelLoad(PathBuf),

    #[error("the path {} is not valid UTF-8", .0.display())]
    NonUtf8Path(PathBuf),

    #[error("download failed: {0}")]
    Http(Box<ureq::Error>),

    #[error("the download has the wrong size: expected {expected} bytes, received {received}")]
    WrongDownloadSize { expected: u64, received: u64 },

    #[error("the downloaded file is corrupted (checksum mismatch)")]
    ChecksumMismatch,

    #[error("the model archive does not contain {0}")]
    IncompleteArchive(&'static str),

    #[error("the operation was cancelled")]
    Cancelled,

    #[error("a background worker stopped unexpectedly")]
    WorkerPanicked,

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<ureq::Error> for Error {
    fn from(error: ureq::Error) -> Self {
        Self::Http(Box::new(error))
    }
}
