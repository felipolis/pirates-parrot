//! Hands a finished transcript to the user: clipboard, and optionally a
//! paste into whichever window has keyboard focus.

use enigo::{Direction, Enigo, Key, Keyboard};
use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::error::AppError;
use crate::settings::OutputMode;

/// Copies `text` and, depending on `mode`, pastes it. Returns whether it was
/// pasted.
///
/// The widget window never takes focus, so the paste lands in the app the
/// user was working in, such as their terminal.
pub fn deliver(app: &AppHandle, text: &str, mode: OutputMode) -> Result<bool, AppError> {
    app.clipboard().write_text(text)?;
    match mode {
        OutputMode::Copy => Ok(false),
        OutputMode::CopyAndPaste => {
            send_paste_shortcut().map_err(|error| AppError::Paste(error.to_string()))?;
            Ok(true)
        }
    }
}

fn send_paste_shortcut() -> Result<(), Box<dyn std::error::Error>> {
    let modifier = if cfg!(target_os = "macos") {
        Key::Meta
    } else {
        Key::Control
    };
    let mut enigo = Enigo::new(&enigo::Settings::default())?;
    enigo.key(modifier, Direction::Press)?;
    let pasted = enigo.key(Key::Unicode('v'), Direction::Click);
    // Always release the modifier, even if the paste key failed, so it is
    // never left stuck down.
    enigo.key(modifier, Direction::Release)?;
    Ok(pasted?)
}
