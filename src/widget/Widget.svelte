<!--
  The floating widget: the parrot alone while idle, plus a speech bubble with
  the live transcript while listening and for a moment after delivering it.
-->
<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { onMount, untrack } from 'svelte';
  import { AppState } from '../lib/app-state.svelte';
  import {
    cancelDictation,
    openParrotMenu,
    openSettings,
    setWidgetExpanded,
    toggleDictation,
  } from '../lib/api/commands';
  import { errorMessage } from '../lib/api/errors';
  import { onEvent } from '../lib/api/events';
  import type { Corner, DictationResult } from '../lib/api/types';
  import type { Mood } from '../lib/parrot/mood';
  import Parrot from '../lib/parrot/Parrot.svelte';
  import { skinById } from '../lib/parrot/skins';
  import SpeechBubble, { type Feedback } from './SpeechBubble.svelte';

  /** Pointer travel, in pixels, that turns a press on the parrot into a drag. */
  const DRAG_THRESHOLD = 4;
  const FEEDBACK_DURATION: Record<Feedback['tone'], number> = {
    success: 1600,
    neutral: 2200,
    error: 4500,
  };

  const app = new AppState();
  let transcript = $state('');
  let level = $state(0);
  let feedback = $state<Feedback | null>(null);
  let listeningSince = $state(0);
  let now = $state(Date.now());
  /** Layout the window currently has; `null` corner means compact. */
  let corner = $state<Corner | null>(null);
  let appliedExpanded: boolean | null = null;
  let resizing = $state(false);
  let feedbackTimer: ReturnType<typeof setTimeout> | undefined;

  const skin = $derived(skinById(app.settings?.parrot));
  const ready = $derived(app.engine.state === 'ready');
  const expanded = $derived(app.phase !== 'idle' || feedback !== null);
  const mood = $derived(currentMood());

  function currentMood(): Mood {
    if (feedback?.tone === 'error' || app.engine.state === 'failed') return 'error';
    if (feedback?.tone === 'success') return 'happy';
    if (app.phase === 'listening') return 'listening';
    if (app.phase === 'finishing') return 'thinking';
    return ready ? 'idle' : 'sleeping';
  }

  function feedbackFor(result: DictationResult): Feedback {
    switch (result.kind) {
      case 'delivered':
        return { tone: 'success', message: result.pasted ? 'Pasted!' : 'Copied to clipboard' };
      case 'empty':
        return { tone: 'neutral', message: "Didn't catch that." };
      case 'failed':
        return { tone: 'error', message: result.message };
    }
  }

  function showFeedback(next: Feedback) {
    clearTimeout(feedbackTimer);
    feedback = next;
    feedbackTimer = setTimeout(() => (feedback = null), FEEDBACK_DURATION[next.tone]);
  }

  /** Shows a failed request in the bubble, where the user is looking. */
  function showFailure(error: unknown) {
    showFeedback({ tone: 'error', message: errorMessage(error) });
  }

  function startListening() {
    clearTimeout(feedbackTimer);
    feedback = null;
    transcript = '';
    level = 0;
    listeningSince = Date.now();
    now = listeningSince;
  }

  onMount(() => {
    const connection = app.connect().catch((error: unknown) => {
      showFailure(error);
      return null;
    });
    const subscriptions = Promise.all([
      onEvent('dictation-phase', (phase) => {
        if (phase === 'listening') startListening();
      }),
      onEvent('transcript', (text) => (transcript = text)),
      onEvent('level', (value) => (level = value)),
      onEvent('dictation-result', (result) => {
        if (result.kind === 'delivered') transcript = result.text;
        showFeedback(feedbackFor(result));
      }),
    ]).catch((error: unknown) => {
      showFailure(error);
      return [];
    });
    return () => {
      clearTimeout(feedbackTimer);
      void connection.then((disconnect) => disconnect?.());
      void subscriptions.then((unlisteners) => unlisteners.forEach((unlisten) => unlisten()));
    };
  });

  // Tick the elapsed-time counter while listening.
  $effect(() => {
    if (app.phase !== 'listening') return;
    const interval = setInterval(() => (now = Date.now()), 250);
    return () => clearInterval(interval);
  });

  // Resize the native window whenever the bubble appears or disappears.
  $effect(() => {
    const wanted = expanded;
    untrack(() => {
      if (wanted !== appliedExpanded) void syncWindowSize();
    });
  });

  /**
   * Brings the native window to the size `expanded` calls for. The content is
   * hidden while the window moves and resizes so the parrot never visibly
   * jumps; changes that arrive meanwhile are applied until the window matches.
   */
  async function syncWindowSize() {
    if (resizing) return;
    resizing = true;
    try {
      while (appliedExpanded !== expanded) {
        const target = expanded;
        const layout = await setWidgetExpanded(target);
        corner = layout.corner;
        appliedExpanded = target;
        await nextFrames(2);
      }
    } catch (error) {
      // The window keeps its last layout; the next change of `expanded`
      // tries again. Showing it in the bubble would need the resize that failed.
      console.error('could not resize the widget:', error);
    } finally {
      resizing = false;
    }
  }

  /**
   * Frames pause while the window is hidden (a hidden parrot vanishes when its
   * bubble closes); the wait then completes when the parrot shows up again.
   */
  function nextFrames(count: number): Promise<void> {
    return new Promise((resolve) => {
      const step = (remaining: number) =>
        remaining === 0 ? resolve() : requestAnimationFrame(() => step(remaining - 1));
      step(count);
    });
  }

  // A press on the parrot either clicks it or, once the pointer moves, drags
  // the whole widget.
  let pressPoint: { x: number; y: number } | null = null;
  let dragged = false;

  function onPointerDown(event: PointerEvent) {
    if (event.button !== 0) return;
    pressPoint = { x: event.screenX, y: event.screenY };
    dragged = false;
  }

  function onPointerMove(event: PointerEvent) {
    if (!pressPoint) return;
    const distance = Math.hypot(event.screenX - pressPoint.x, event.screenY - pressPoint.y);
    if (distance < DRAG_THRESHOLD) return;
    pressPoint = null;
    dragged = true;
    void getCurrentWindow().startDragging();
  }

  function onParrotClick() {
    pressPoint = null;
    if (dragged) {
      dragged = false;
      return;
    }
    (ready ? toggleDictation() : openSettings()).catch(showFailure);
  }

  function onParrotContextMenu(event: MouseEvent) {
    event.preventDefault();
    openParrotMenu().catch(showFailure);
  }

  function parrotHint(): string {
    switch (app.engine.state) {
      case 'ready':
        return app.phase === 'listening' ? 'Click to finish' : 'Click to start talking';
      case 'installing':
        return 'Downloading the speech model…';
      case 'loading':
        return 'Waking up…';
      case 'failed':
        return 'Something is wrong. Click to open settings.';
      case 'notInstalled':
        return 'Click to download the speech model';
    }
  }
</script>

<main class="widget {corner ?? 'compact'}" class:resizing>
  {#if corner}
    <SpeechBubble
      {corner}
      phase={app.phase}
      {transcript}
      {feedback}
      elapsedMs={now - listeningSince}
      onFinish={() => toggleDictation().catch(showFailure)}
      onDiscard={() => cancelDictation().catch(showFailure)}
    />
  {/if}

  <button
    class="parrot-button"
    title={parrotHint()}
    aria-label={parrotHint()}
    onpointerdown={onPointerDown}
    onpointermove={onPointerMove}
    onclick={onParrotClick}
    oncontextmenu={onParrotContextMenu}
  >
    <Parrot {skin} {mood} {level} size={80} />
  </button>
</main>

<style>
  :global(html),
  :global(body) {
    height: 100%;
    background: transparent;
    overflow: hidden;
    user-select: none;
  }

  .widget {
    position: relative;
    width: 100vw;
    height: 100vh;
  }

  .resizing {
    opacity: 0;
  }

  .parrot-button {
    position: absolute;
    width: 96px;
    height: 96px;
    display: grid;
    place-items: center;
    padding: 0;
    border: none;
    background: none;
  }

  .compact .parrot-button,
  .topLeft .parrot-button {
    top: 0;
    left: 0;
  }

  .topRight .parrot-button {
    top: 0;
    right: 0;
  }

  .bottomLeft .parrot-button {
    bottom: 0;
    left: 0;
  }

  .bottomRight .parrot-button {
    bottom: 0;
    right: 0;
  }
</style>
