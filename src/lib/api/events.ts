// Typed subscriptions to the events emitted by `src-tauri/src/events.rs`.

import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { DictationPhase, DictationResult, EngineStatus, Settings } from './types';

interface EventPayloads {
  'engine-status': EngineStatus;
  'dictation-phase': DictationPhase;
  transcript: string;
  level: number;
  'dictation-result': DictationResult;
  'settings-changed': Settings;
  /** Why the shortcut cannot be registered, or `null` once it works again. */
  'shortcut-problem': string | null;
}

export type BackendEvent = keyof EventPayloads;

export function onEvent<E extends BackendEvent>(
  event: E,
  handler: (payload: EventPayloads[E]) => void,
): Promise<UnlistenFn> {
  return listen<EventPayloads[E]>(event, ({ payload }) => handler(payload));
}
