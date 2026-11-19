//! System tray icon: the way back to the parrot and the settings when the
//! widget is hidden.

use tauri::AppHandle;
use tauri::tray::TrayIconBuilder;

use crate::error::AppError;
use crate::menu;

pub fn create(app: &AppHandle) -> Result<(), AppError> {
    // Clicks on the menu are handled by `menu::handle_event`, registered for
    // the whole app.
    let mut builder = TrayIconBuilder::with_id("main")
        .tooltip("Pirate's Parrot")
        .menu(&menu::tray_menu(app)?)
        .show_menu_on_left_click(true);
    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }
    builder.build(app)?;
    Ok(())
}
