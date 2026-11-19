//! The settings window, created the first time it is opened.

use tauri::utils::config::Color;
use tauri::{AppHandle, Manager, Theme, WebviewUrl, WebviewWindowBuilder, WindowEvent};

use super::widget;
use crate::error::AppError;
use crate::shortcut;

const LABEL: &str = "settings";

/// The settings page's sea color (`--sea-deep` in `src/styles/base.css`),
/// painted before the page loads so the window never flashes white.
const SEA_AT_NIGHT: Color = Color(7, 18, 28, 255);
const SEA_BY_DAY: Color = Color(27, 87, 99, 255);

/// Shows the settings window, creating it on first use.
pub fn open(app: &AppHandle) -> Result<(), AppError> {
    if let Some(window) = app.get_webview_window(LABEL) {
        window.unminimize()?;
        window.show()?;
        window.set_focus()?;
        return Ok(());
    }
    let window = WebviewWindowBuilder::new(app, LABEL, WebviewUrl::App("settings.html".into()))
        .title("Pirate's Parrot")
        .inner_size(640.0, 820.0)
        .min_inner_size(480.0, 560.0)
        .background_color(sea_color(app))
        .center()
        .build()?;

    // Closing the window while it records a new shortcut must not leave the
    // app without one.
    let app = app.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::Destroyed = event {
            shortcut::resume_if_suspended(&app);
        }
    });
    Ok(())
}

/// Matches the system theme, which the always-present widget window reports.
fn sea_color(app: &AppHandle) -> Color {
    let theme = app
        .get_webview_window(widget::LABEL)
        .and_then(|window| window.theme().ok());
    match theme {
        Some(Theme::Light) => SEA_BY_DAY,
        _ => SEA_AT_NIGHT,
    }
}
