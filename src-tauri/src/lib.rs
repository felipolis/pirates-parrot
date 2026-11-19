//! Pirate's Parrot desktop app: a floating parrot that turns speech into text
//! for whatever app you are working in.
//!
//! The speech engine lives in the `parrot-core` crate. This crate is the
//! desktop shell around it: windows, tray icon, global shortcut, clipboard and
//! the commands and events the Svelte frontend talks to.

mod commands;
mod dictation;
mod engine;
mod error;
mod events;
mod menu;
mod output;
mod persistence;
mod settings;
mod shortcut;
mod tray;
mod windows;

use parrot_core::ModelStore;
use tauri::{AppHandle, Manager};

use crate::dictation::DictationController;
use crate::engine::{Engine, EngineStatus};
use crate::error::AppError;
use crate::settings::SettingsFile;
use crate::shortcut::ShortcutRegistration;
use crate::windows::widget::{PlacementFile, WidgetState};

pub fn run() {
    tauri::Builder::default()
        // Must be registered first so a second launch exits before doing any
        // work; it brings the settings window of the running instance forward.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Err(error) = windows::settings::open(app) {
                log::error!("could not open settings: {error}");
            }
        }))
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| Ok(setup(app.handle())?))
        .invoke_handler(tauri::generate_handler![
            commands::get_snapshot,
            commands::list_models,
            commands::list_languages,
            commands::list_microphones,
            commands::update_settings,
            commands::toggle_dictation,
            commands::cancel_dictation,
            commands::install_model,
            commands::cancel_model_install,
            commands::remove_model,
            commands::suspend_shortcut,
            commands::resume_shortcut,
            commands::set_widget_expanded,
            commands::open_settings,
            commands::open_parrot_menu,
        ])
        .run(tauri::generate_context!())
        .expect("Pirate's Parrot failed to start");
}

fn setup(app: &AppHandle) -> Result<(), AppError> {
    let config_dir = app.path().app_config_dir()?;
    let models_dir = app.path().app_local_data_dir()?.join("models");

    let settings_file = SettingsFile::load(config_dir.join("settings.json"));
    let settings = settings_file.get();
    app.manage(settings_file);
    app.manage(PlacementFile::load(config_dir.join("widget.json")));
    app.manage(Engine::new(ModelStore::new(models_dir), settings.model));
    app.manage(DictationController::new(app)?);
    app.manage(WidgetState::default());
    app.manage(ShortcutRegistration::default());

    windows::widget::create(app)?;
    tray::create(app)?;
    app.on_menu_event(menu::handle_event);
    if let Err(error) = shortcut::register(app, &settings.shortcut, settings.shortcut_mode) {
        log::warn!("shortcut unavailable: {error}");
    }

    let engine = app.state::<Engine>();
    engine.activate(app, settings.model);
    // First launch: the model still has to be downloaded, which happens from
    // the settings window.
    if engine.status() == EngineStatus::NotInstalled {
        windows::settings::open(app)?;
    }
    Ok(())
}
