//! User preferences and the file they are saved in.

use parrot_core::{Language, ModelId};
use serde::{Deserialize, Serialize};

use crate::persistence::JsonFile;

/// The settings, as stored in `settings.json`.
pub type SettingsFile = JsonFile<Settings>;

/// User preferences, edited in the settings window.
///
/// Every field has a default, so settings files written by older versions
/// keep loading after new fields are added.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    /// Global shortcut in Tauri accelerator syntax, e.g. `CommandOrControl+Shift+Space`.
    pub shortcut: String,
    pub shortcut_mode: ShortcutMode,
    pub language: Language,
    pub model: ModelId,
    /// Microphone identifier, or `None` for the system default.
    pub microphone: Option<String>,
    pub output: OutputMode,
    /// Identifier of the parrot skin shown in the widget.
    pub parrot: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            shortcut: "CommandOrControl+Shift+Space".to_owned(),
            shortcut_mode: ShortcutMode::default(),
            language: Language::default(),
            model: ModelId::default(),
            microphone: None,
            output: OutputMode::default(),
            parrot: "captain-scarlet".to_owned(),
        }
    }
}

/// How the global shortcut controls a dictation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ShortcutMode {
    /// Press once to start listening, press again to stop.
    #[default]
    Toggle,
    /// Listen only while the shortcut is held down.
    Hold,
}

/// What happens with the transcript when a dictation ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OutputMode {
    /// Copy it to the clipboard.
    Copy,
    /// Copy it and paste it into the window that has focus.
    #[default]
    CopyAndPaste,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_fields_take_defaults() {
        let settings: Settings = serde_json::from_str(r#"{ "shortcutMode": "hold" }"#).unwrap();

        assert_eq!(settings.shortcut_mode, ShortcutMode::Hold);
        assert_eq!(settings.shortcut, Settings::default().shortcut);
    }

    #[test]
    fn round_trips_through_json() {
        let settings = Settings {
            language: Language::PortugueseBrazil,
            microphone: Some("wasapi:{0.0.1}".to_owned()),
            output: OutputMode::Copy,
            ..Settings::default()
        };

        let json = serde_json::to_string(&settings).unwrap();

        assert!(json.contains(r#""language":"pt-BR""#));
        assert_eq!(serde_json::from_str::<Settings>(&json).unwrap(), settings);
    }
}
