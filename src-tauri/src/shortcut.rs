//! The global keyboard shortcut that starts and stops dictation from any app.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, PoisonError};

use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::dictation::{DictationController, Request};
use crate::error::AppError;
use crate::events;
use crate::settings::{SettingsFile, ShortcutMode};

/// Tracks the shortcut's registration: why the configured shortcut could not
/// be registered, so the settings window can tell the user instead of failing
/// silently, and whether it is suspended while the user records a new one.
#[derive(Debug, Default)]
pub struct ShortcutRegistration {
    problem: Mutex<Option<String>>,
    suspended: AtomicBool,
}

impl ShortcutRegistration {
    pub fn problem(&self) -> Option<String> {
        self.problem
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    fn record(&self, app: &AppHandle, result: &Result<(), AppError>) {
        self.suspended.store(false, Ordering::Relaxed);
        let problem = result.as_ref().err().map(ToString::to_string);
        let mut current = self.problem.lock().unwrap_or_else(PoisonError::into_inner);
        if *current != problem {
            events::shortcut_problem(app, problem.as_deref());
            *current = problem;
        }
    }
}

/// Replaces the registered shortcut with `accelerator`, e.g.
/// `CommandOrControl+Shift+Space`.
pub fn register(app: &AppHandle, accelerator: &str, mode: ShortcutMode) -> Result<(), AppError> {
    let result = try_register(app, accelerator, mode);
    app.state::<ShortcutRegistration>().record(app, &result);
    result
}

/// Unregisters the shortcut while the user records a new one, so pressing the
/// current combination does not start a dictation.
pub fn suspend(app: &AppHandle) -> Result<(), AppError> {
    app.global_shortcut().unregister_all()?;
    app.state::<ShortcutRegistration>()
        .suspended
        .store(true, Ordering::Relaxed);
    Ok(())
}

/// Registers the shortcut from the saved settings again after [`suspend`].
pub fn resume(app: &AppHandle) -> Result<(), AppError> {
    let settings = app.state::<SettingsFile>().get();
    register(app, &settings.shortcut, settings.shortcut_mode)
}

/// Brings back a shortcut left suspended, for when the window that was
/// recording a new one goes away before it could resume it.
pub fn resume_if_suspended(app: &AppHandle) {
    if !app
        .state::<ShortcutRegistration>()
        .suspended
        .load(Ordering::Relaxed)
    {
        return;
    }
    if let Err(error) = resume(app) {
        log::error!("could not restore the shortcut: {error}");
    }
}

fn try_register(app: &AppHandle, accelerator: &str, mode: ShortcutMode) -> Result<(), AppError> {
    let shortcut: Shortcut = accelerator
        .parse()
        .map_err(|_| AppError::InvalidShortcut(accelerator.to_owned()))?;
    let manager = app.global_shortcut();
    manager.unregister_all()?;
    manager.on_shortcut(shortcut, move |app, _, event| {
        let request = match (mode, event.state) {
            (ShortcutMode::Toggle, ShortcutState::Pressed) => Request::Toggle,
            (ShortcutMode::Hold, ShortcutState::Pressed) => Request::Start,
            (ShortcutMode::Hold, ShortcutState::Released) => Request::Finish,
            (ShortcutMode::Toggle, ShortcutState::Released) => return,
        };
        app.state::<DictationController>().request(request);
    })?;
    Ok(())
}
