//! Errors of the desktop app, as reported to the frontend.

use serde::{Serialize, Serializer};

/// Errors surfaced by the app layer. Commands return them to the frontend,
/// where they arrive as their human-readable message.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Core(#[from] parrot_core::Error),

    #[error("the speech model is not ready yet")]
    ModelNotReady,

    #[error("a model download is already in progress")]
    InstallInProgress,

    #[error("the model in use cannot be removed; switch to another model first")]
    ModelInUse,

    #[error("\"{0}\" is not a valid shortcut")]
    InvalidShortcut(String),

    #[error("could not register the shortcut: {0}")]
    Shortcut(#[from] tauri_plugin_global_shortcut::Error),

    #[error("could not use the clipboard: {0}")]
    Clipboard(#[from] tauri_plugin_clipboard_manager::Error),

    #[error("no screen is available to show the widget")]
    NoScreen,

    #[error("could not paste into the active window: {0}")]
    Paste(String),

    #[error(transparent)]
    Tauri(#[from] tauri::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
