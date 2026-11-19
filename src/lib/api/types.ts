// Mirrors of the Rust types exchanged with the backend. Keep in sync with
// `src-tauri/src` (settings.rs, engine.rs, events.rs, commands.rs) and
// `crates/parrot-core/src`.

/** Locale code such as `pt-BR`, or `auto`. */
export type LanguageCode = string;

export type ModelId = 'nemotron-fast' | 'nemotron-balanced' | 'nemotron-accurate';

export type ShortcutMode = 'toggle' | 'hold';

export type OutputMode = 'copy' | 'copyAndPaste';

export interface Settings {
  /** Tauri accelerator, e.g. `CommandOrControl+Shift+Space`. */
  shortcut: string;
  shortcutMode: ShortcutMode;
  language: LanguageCode;
  model: ModelId;
  /** Microphone identifier, or `null` for the system default. */
  microphone: string | null;
  output: OutputMode;
  /** Parrot skin identifier. */
  parrot: string;
}

export type InstallProgress =
  { stage: 'downloading'; downloaded: number; total: number } | { stage: 'extracting' };

export type EngineStatus =
  | { state: 'notInstalled' }
  | { state: 'installing'; progress: InstallProgress }
  | { state: 'loading' }
  | { state: 'ready' }
  | { state: 'failed'; message: string };

export type DictationPhase = 'idle' | 'listening' | 'finishing';

export type DictationResult =
  | { kind: 'delivered'; text: string; pasted: boolean }
  | { kind: 'empty' }
  | { kind: 'failed'; message: string };

export interface Snapshot {
  settings: Settings;
  engine: EngineStatus;
  phase: DictationPhase;
  shortcutProblem: string | null;
}

export interface ModelInfo {
  id: ModelId;
  name: string;
  description: string;
  chunkMs: number;
  downloadBytes: number;
  installed: boolean;
}

export interface LanguageOption {
  code: LanguageCode;
  name: string;
}

export interface InputDevice {
  id: string;
  name: string;
  isDefault: boolean;
}

export type Corner = 'topLeft' | 'topRight' | 'bottomLeft' | 'bottomRight';

export interface WidgetLayout {
  /** Corner holding the parrot while expanded; `null` when compact. */
  corner: Corner | null;
}
