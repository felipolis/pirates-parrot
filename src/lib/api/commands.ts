// Typed wrappers around the backend commands in `src-tauri/src/commands.rs`.

import { invoke } from '@tauri-apps/api/core';
import type {
  InputDevice,
  LanguageOption,
  ModelId,
  ModelInfo,
  Settings,
  Snapshot,
  WidgetLayout,
} from './types';

export const getSnapshot = () => invoke<Snapshot>('get_snapshot');

export const listModels = () => invoke<ModelInfo[]>('list_models');

export const listLanguages = () => invoke<LanguageOption[]>('list_languages');

export const listMicrophones = () => invoke<InputDevice[]>('list_microphones');

export const updateSettings = (settings: Settings) =>
  invoke<Settings>('update_settings', { settings });

export const toggleDictation = (): Promise<void> => invoke('toggle_dictation');

export const cancelDictation = (): Promise<void> => invoke('cancel_dictation');

export const installModel = (): Promise<void> => invoke('install_model');

export const cancelModelInstall = (): Promise<void> => invoke('cancel_model_install');

export const removeModel = (model: ModelId): Promise<void> => invoke('remove_model', { model });

export const suspendShortcut = (): Promise<void> => invoke('suspend_shortcut');

export const resumeShortcut = (): Promise<void> => invoke('resume_shortcut');

export const setWidgetExpanded = (expanded: boolean) =>
  invoke<WidgetLayout>('set_widget_expanded', { expanded });

export const openSettings = (): Promise<void> => invoke('open_settings');

export const openParrotMenu = (): Promise<void> => invoke('open_parrot_menu');
