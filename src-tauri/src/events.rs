//! Events pushed from the backend to every window. Names and payloads are
//! mirrored in `src/lib/api/events.ts`.

use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::dictation::DictationPhase;
use crate::engine::EngineStatus;
use crate::settings::Settings;

pub const ENGINE_STATUS: &str = "engine-status";
pub const DICTATION_PHASE: &str = "dictation-phase";
pub const TRANSCRIPT: &str = "transcript";
pub const LEVEL: &str = "level";
pub const DICTATION_RESULT: &str = "dictation-result";
pub const SETTINGS_CHANGED: &str = "settings-changed";
pub const SHORTCUT_PROBLEM: &str = "shortcut-problem";

/// How a dictation ended.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DictationResult {
    /// The text was copied, and pasted if the settings ask for it.
    Delivered {
        text: String,
        pasted: bool,
    },
    /// Nothing intelligible was said.
    Empty,
    Failed {
        message: String,
    },
}

/// Sends an event to every window. Failing to notify the UI is logged rather
/// than propagated: it must never interrupt the work that produced the event.
pub fn emit(app: &AppHandle, event: &str, payload: impl Serialize + Clone) {
    if let Err(error) = app.emit(event, payload) {
        log::warn!("could not emit {event}: {error}");
    }
}

pub fn engine_status(app: &AppHandle, status: &EngineStatus) {
    emit(app, ENGINE_STATUS, status);
}

pub fn dictation_phase(app: &AppHandle, phase: DictationPhase) {
    emit(app, DICTATION_PHASE, phase);
}

pub fn transcript(app: &AppHandle, text: &str) {
    emit(app, TRANSCRIPT, text);
}

pub fn level(app: &AppHandle, level: f32) {
    emit(app, LEVEL, level);
}

pub fn dictation_result(app: &AppHandle, result: &DictationResult) {
    emit(app, DICTATION_RESULT, result);
}

pub fn settings_changed(app: &AppHandle, settings: &Settings) {
    emit(app, SETTINGS_CHANGED, settings);
}

/// Why the shortcut cannot be registered, or `None` once it works again.
pub fn shortcut_problem(app: &AppHandle, problem: Option<&str>) {
    emit(app, SHORTCUT_PROBLEM, problem);
}
