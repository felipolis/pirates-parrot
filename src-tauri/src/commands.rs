//! Commands the frontend can invoke. Their TypeScript counterparts live in
//! `src/lib/api/commands.ts`.

use parrot_core::{InputDevice, Language, ModelId};
use serde::Serialize;
use tauri::{AppHandle, Manager, State, Window};

use crate::dictation::{DictationController, DictationPhase, Request};
use crate::engine::{Engine, EngineStatus, ModelInfo};
use crate::error::AppError;
use crate::events;
use crate::menu;
use crate::settings::{Settings, SettingsFile};
use crate::shortcut::{self, ShortcutRegistration};
use crate::windows::{self, widget};

/// Everything a window needs to render its first frame.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    settings: Settings,
    engine: EngineStatus,
    phase: DictationPhase,
    shortcut_problem: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LanguageOption {
    code: Language,
    name: &'static str,
}

#[tauri::command]
pub fn get_snapshot(
    settings: State<'_, SettingsFile>,
    engine: State<'_, Engine>,
    dictation: State<'_, DictationController>,
    registration: State<'_, ShortcutRegistration>,
) -> Snapshot {
    Snapshot {
        settings: settings.get(),
        engine: engine.status(),
        phase: dictation.phase(),
        shortcut_problem: registration.problem(),
    }
}

#[tauri::command]
pub fn list_models(engine: State<'_, Engine>) -> Vec<ModelInfo> {
    engine.models()
}

#[tauri::command]
pub fn list_languages() -> Vec<LanguageOption> {
    Language::ALL
        .iter()
        .map(|&code| LanguageOption {
            code,
            name: code.display_name(),
        })
        .collect()
}

#[tauri::command]
pub async fn list_microphones() -> Result<Vec<InputDevice>, AppError> {
    Ok(parrot_core::list_input_devices()?)
}

/// Saves new settings and applies whatever changed. If the new shortcut cannot
/// be registered nothing is saved and the previous shortcut stays active, so
/// the stored settings always describe what is actually in effect.
#[tauri::command]
pub fn update_settings(app: AppHandle, settings: Settings) -> Result<Settings, AppError> {
    let file = app.state::<SettingsFile>();
    let previous = file.get();

    let shortcut_changed =
        settings.shortcut != previous.shortcut || settings.shortcut_mode != previous.shortcut_mode;
    let applied = if shortcut_changed {
        shortcut::register(&app, &settings.shortcut, settings.shortcut_mode)
    } else {
        Ok(())
    }
    .and_then(|()| file.set(settings.clone()));

    if let Err(error) = applied {
        if shortcut_changed {
            restore_shortcut(&app, &previous);
        }
        return Err(error);
    }

    if settings.model != previous.model {
        app.state::<Engine>().activate(&app, settings.model);
    }
    events::settings_changed(&app, &settings);
    Ok(settings)
}

fn restore_shortcut(app: &AppHandle, previous: &Settings) {
    if let Err(error) = shortcut::register(app, &previous.shortcut, previous.shortcut_mode) {
        log::error!("could not restore the previous shortcut: {error}");
    }
}

#[tauri::command]
pub fn toggle_dictation(dictation: State<'_, DictationController>) {
    dictation.request(Request::Toggle);
}

#[tauri::command]
pub fn cancel_dictation(dictation: State<'_, DictationController>) {
    dictation.request(Request::Cancel);
}

#[tauri::command]
pub fn install_model(app: AppHandle, engine: State<'_, Engine>) -> Result<(), AppError> {
    engine.install_active(&app)
}

#[tauri::command]
pub fn cancel_model_install(engine: State<'_, Engine>) {
    engine.cancel_install();
}

#[tauri::command]
pub fn suspend_shortcut(app: AppHandle) -> Result<(), AppError> {
    shortcut::suspend(&app)
}

/// Registers the saved shortcut again. A failure is reported as the shortcut
/// problem, which the settings window shows.
#[tauri::command]
pub fn resume_shortcut(app: AppHandle) {
    if let Err(error) = shortcut::resume(&app) {
        log::warn!("could not resume the shortcut: {error}");
    }
}

// The commands below are async so they run off the main thread: they touch
// windows (creating one from the main thread deadlocks on Windows) or do slow
// work, like deleting hundreds of megabytes or enumerating audio devices.

#[tauri::command]
pub async fn remove_model(app: AppHandle, model: ModelId) -> Result<(), AppError> {
    app.state::<Engine>().remove(model)
}

#[tauri::command]
pub async fn set_widget_expanded(
    app: AppHandle,
    expanded: bool,
) -> Result<widget::WidgetLayout, AppError> {
    widget::set_expanded(&app, expanded)
}

#[tauri::command]
pub async fn open_settings(app: AppHandle) -> Result<(), AppError> {
    windows::settings::open(&app)
}

/// Shows the parrot's right-click menu under the pointer.
#[tauri::command]
pub async fn open_parrot_menu(app: AppHandle, window: Window) -> Result<(), AppError> {
    window.popup_menu(&menu::parrot_menu(&app)?)?;
    Ok(())
}
