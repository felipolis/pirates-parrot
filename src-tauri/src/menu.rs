//! The tray menu and the parrot's right-click menu, and what their items do.

use tauri::menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tauri::{AppHandle, Manager, Wry};

use crate::error::AppError;
use crate::windows::{self, widget};

const SHOW_PARROT: &str = "show-parrot";
const HIDE_PARROT: &str = "hide-parrot";
const SETTINGS: &str = "settings";
const QUIT: &str = "quit";

/// The tray's "Show parrot" check mark, kept in step with the parrot however
/// it gets hidden or shown.
struct ParrotToggle(CheckMenuItem<Wry>);

/// Builds the tray menu. Call once: the menu's check mark is registered as
/// app state.
pub fn tray_menu(app: &AppHandle) -> Result<Menu<Wry>, AppError> {
    let show_parrot = CheckMenuItem::with_id(
        app,
        SHOW_PARROT,
        "Show parrot",
        true,
        !widget::is_hidden(app),
        None::<&str>,
    )?;
    let menu = Menu::with_items(
        app,
        &[
            &show_parrot,
            &settings_item(app)?,
            &PredefinedMenuItem::separator(app)?,
            &quit_item(app)?,
        ],
    )?;
    app.manage(ParrotToggle(show_parrot));
    Ok(menu)
}

/// Builds the menu shown when the parrot is right-clicked.
pub fn parrot_menu(app: &AppHandle) -> Result<Menu<Wry>, AppError> {
    Ok(Menu::with_items(
        app,
        &[
            &MenuItem::with_id(app, HIDE_PARROT, "Hide parrot", true, None::<&str>)?,
            &settings_item(app)?,
            &PredefinedMenuItem::separator(app)?,
            &quit_item(app)?,
        ],
    )?)
}

/// Handles a click on an item of any of the app's menus.
pub fn handle_event(app: &AppHandle, event: MenuEvent) {
    let result = match event.id().as_ref() {
        SHOW_PARROT => set_parrot_hidden(app, !widget::is_hidden(app)),
        HIDE_PARROT => set_parrot_hidden(app, true),
        SETTINGS => windows::settings::open(app),
        QUIT => {
            app.exit(0);
            Ok(())
        }
        _ => Ok(()),
    };
    if let Err(error) = result {
        log::error!("menu action failed: {error}");
    }
}

fn set_parrot_hidden(app: &AppHandle, hidden: bool) -> Result<(), AppError> {
    widget::set_hidden(app, hidden)?;
    app.state::<ParrotToggle>().0.set_checked(!hidden)?;
    Ok(())
}

fn settings_item(app: &AppHandle) -> Result<MenuItem<Wry>, AppError> {
    Ok(MenuItem::with_id(
        app,
        SETTINGS,
        "Settings…",
        true,
        None::<&str>,
    )?)
}

fn quit_item(app: &AppHandle) -> Result<MenuItem<Wry>, AppError> {
    Ok(MenuItem::with_id(
        app,
        QUIT,
        "Quit Pirate's Parrot",
        true,
        None::<&str>,
    )?)
}
