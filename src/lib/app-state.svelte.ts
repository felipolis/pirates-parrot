import { getSnapshot } from './api/commands';
import { onEvent } from './api/events';
import type { DictationPhase, EngineStatus, Settings } from './api/types';

/**
 * The backend state every window mirrors: settings, speech model status and
 * dictation phase. Kept current by backend events.
 */
export class AppState {
  settings = $state<Settings | null>(null);
  engine = $state<EngineStatus>({ state: 'loading' });
  phase = $state<DictationPhase>('idle');
  shortcutProblem = $state<string | null>(null);

  /**
   * Subscribes to backend updates, then loads the current state. Subscribing
   * first means no update can slip in between. Returns a cleanup function;
   * if loading fails, the subscriptions are removed and the error rethrown.
   */
  async connect(): Promise<() => void> {
    const unlisteners = await Promise.all([
      onEvent('engine-status', (status) => (this.engine = status)),
      onEvent('dictation-phase', (phase) => (this.phase = phase)),
      onEvent('settings-changed', (settings) => (this.settings = settings)),
      onEvent('shortcut-problem', (problem) => (this.shortcutProblem = problem)),
    ]);
    const disconnect = () => unlisteners.forEach((unlisten) => unlisten());

    try {
      const snapshot = await getSnapshot();
      this.settings = snapshot.settings;
      this.engine = snapshot.engine;
      this.phase = snapshot.phase;
      this.shortcutProblem = snapshot.shortcutProblem;
    } catch (error) {
      disconnect();
      throw error;
    }
    return disconnect;
  }
}
