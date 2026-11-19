<!--
  The parrot's speech bubble: live transcript while listening, then a short
  confirmation of what happened to the text.
-->
<script lang="ts" module>
  export interface Feedback {
    tone: 'success' | 'neutral' | 'error';
    message: string;
  }
</script>

<script lang="ts">
  import type { Corner, DictationPhase } from '../lib/api/types';

  interface Props {
    /** Corner of the widget holding the parrot; the bubble's tail points there. */
    corner: Corner;
    phase: DictationPhase;
    transcript: string;
    feedback: Feedback | null;
    elapsedMs: number;
    onFinish: () => void;
    onDiscard: () => void;
  }

  let { corner, phase, transcript, feedback, elapsedMs, onFinish, onDiscard }: Props = $props();

  let textElement = $state<HTMLParagraphElement>();

  const status = $derived(
    feedback?.message ?? (phase === 'finishing' ? 'Writing it down…' : 'Listening'),
  );
  const tone = $derived(feedback?.tone ?? (phase === 'listening' ? 'live' : 'neutral'));

  // Keep the newest words in view as the transcript grows.
  $effect(() => {
    void transcript;
    textElement?.scrollTo({ top: textElement.scrollHeight });
  });

  function formatElapsed(milliseconds: number): string {
    const seconds = Math.max(0, Math.floor(milliseconds / 1000));
    return `${Math.floor(seconds / 60)}:${String(seconds % 60).padStart(2, '0')}`;
  }
</script>

<section class="bubble parchment {corner}" aria-live="polite">
  <header class="status {tone}">
    <span class="dot" aria-hidden="true"></span>
    <span class="label">{status}</span>
    {#if phase === 'listening' && !feedback}
      <span class="elapsed">{formatElapsed(elapsedMs)}</span>
      <div class="actions">
        <button class="action discard" title="Discard" aria-label="Discard" onclick={onDiscard}>
          <svg viewBox="0 0 16 16" aria-hidden="true"><path d="M4 4l8 8M12 4l-8 8" /></svg>
        </button>
        <button class="action finish" title="Done" aria-label="Done" onclick={onFinish}>
          <svg viewBox="0 0 16 16" aria-hidden="true"><path d="M3 8.5l3.2 3L13 4.5" /></svg>
        </button>
      </div>
    {/if}
  </header>

  <p class="text" class:placeholder={!transcript} bind:this={textElement}>
    {transcript || (phase === 'listening' ? 'Speak up, I’m all ears…' : '')}
  </p>
</section>

<style>
  .bubble {
    position: absolute;
    left: 8px;
    right: 8px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 10px 14px 12px;
    border: 1.5px solid var(--ink);
    border-radius: 16px;
    box-shadow:
      inset 0 0 0 3px rgb(51 36 26 / 8%),
      0 6px 16px rgb(4 18 28 / 40%);
  }

  /* The bubble sits on the side of the parrot facing the screen center. */
  .bottomLeft,
  .bottomRight {
    top: 8px;
    bottom: 88px;
  }

  .topLeft,
  .topRight {
    top: 88px;
    bottom: 8px;
  }

  /* Tail pointing at the parrot's head. */
  .bubble::after {
    content: '';
    position: absolute;
    width: 14px;
    height: 14px;
    border: 1.5px solid var(--ink);
    /* The bubble's edge is darkened by its grain and inner shadow. */
    background: color-mix(in srgb, var(--parchment) 80%, var(--ink-soft));
    transform: rotate(45deg);
  }

  /*
    Only the half of the diamond outside the bubble is drawn, plus a sliver
    that covers the bubble's border so the tail joins it seamlessly.
  */
  .bottomLeft::after,
  .bottomRight::after {
    bottom: -8.5px;
    border-top: none;
    border-left: none;
    clip-path: polygon(100% 0, 100% 100%, 0 100%, 0 82%, 82% 0);
  }

  .topLeft::after,
  .topRight::after {
    top: -8.5px;
    border-bottom: none;
    border-right: none;
    clip-path: polygon(0 0, 100% 0, 100% 18%, 18% 100%, 0 100%);
  }

  .bottomLeft::after,
  .topLeft::after {
    left: 33px;
  }

  .bottomRight::after,
  .topRight::after {
    right: 33px;
  }

  .status {
    display: flex;
    align-items: center;
    gap: 8px;
    min-height: 24px;
    padding-bottom: 5px;
    border-bottom: 1px solid rgb(51 36 26 / 25%);
    color: var(--ink-soft);
    font-family: var(--font-print-caps);
    font-size: 15px;
  }

  .dot {
    width: 9px;
    height: 9px;
    flex: none;
    border-radius: 50%;
    background: var(--ink-faint);
  }

  .live .dot {
    background: var(--scarlet);
    box-shadow: 0 0 0 3px rgb(181 53 42 / 20%);
    animation: pulse 1.2s ease-in-out infinite;
  }

  .success {
    color: var(--kelp);
  }

  .success .dot {
    background: var(--kelp);
  }

  .error {
    color: var(--scarlet);
  }

  .error .dot {
    background: var(--scarlet);
  }

  .label {
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  .elapsed {
    color: var(--ink-faint);
    font-family: var(--font-body);
    font-size: 12px;
    font-variant-numeric: tabular-nums;
    font-weight: 700;
  }

  .actions {
    display: flex;
    gap: 5px;
    margin-left: auto;
  }

  .action {
    display: grid;
    place-items: center;
    width: 26px;
    height: 26px;
    padding: 0;
    border: 1.5px solid var(--ink);
    border-radius: 50%;
    background: rgb(255 250 235 / 60%);
    color: var(--ink);
  }

  .action:hover {
    background: var(--parchment-light);
  }

  .finish,
  .finish:hover {
    background: var(--teal-ink);
    box-shadow: 0 2px 0 var(--ink);
    color: var(--parchment-light);
  }

  .action svg {
    width: 12px;
    height: 12px;
    fill: none;
    stroke: currentColor;
    stroke-width: 2.2;
    stroke-linecap: round;
    stroke-linejoin: round;
  }

  .text {
    flex: 1;
    margin: 0;
    overflow-y: auto;
    scrollbar-width: none;
    font-size: 14px;
    font-weight: 600;
    line-height: 1.45;
    word-wrap: break-word;
  }

  .placeholder {
    color: var(--ink-faint);
    font-family: var(--font-print);
    font-size: 16px;
    font-style: italic;
    font-weight: 400;
  }

  @keyframes pulse {
    50% {
      opacity: 0.4;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .live .dot {
      animation: none;
    }
  }
</style>
