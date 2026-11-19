<!--
  Shows the global shortcut and records a new one. While recording, the
  current shortcut is suspended so pressing it does not start a dictation.
-->
<script lang="ts">
  import { resumeShortcut, suspendShortcut } from '../lib/api/commands';
  import { acceleratorFromEvent } from '../lib/shortcut';
  import Keycaps from '../lib/ui/Keycaps.svelte';

  interface Props {
    accelerator: string;
    /** Saves the new shortcut. Resolves to `false` if it could not be registered. */
    onChange: (accelerator: string) => Promise<boolean>;
    /** Reports a failure to pause or restore the shortcut. */
    onError: (error: unknown) => void;
  }

  let { accelerator, onChange, onError }: Props = $props();

  let recording = $state(false);

  async function startRecording() {
    try {
      await suspendShortcut();
      recording = true;
    } catch (error) {
      onError(error);
    }
  }

  async function stopRecording(recorded: string | null) {
    if (!recording) return;
    recording = false;
    // Saving a different shortcut registers it; in every other case the
    // suspended one has to be put back.
    const saved = recorded !== null && recorded !== accelerator && (await onChange(recorded));
    if (saved) return;
    try {
      await resumeShortcut();
    } catch (error) {
      onError(error);
    }
  }

  function onKeyDown(event: KeyboardEvent) {
    if (!recording) return;
    event.preventDefault();
    event.stopPropagation();
    const noModifiers = !event.ctrlKey && !event.metaKey && !event.altKey && !event.shiftKey;
    if (event.key === 'Escape' && noModifiers) {
      void stopRecording(null);
      return;
    }
    const recorded = acceleratorFromEvent(event);
    if (recorded) void stopRecording(recorded);
  }
</script>

<svelte:window onkeydown={onKeyDown} />

<button
  class="shortcut"
  class:recording
  onclick={() => void (recording ? stopRecording(null) : startRecording())}
  onblur={() => void stopRecording(null)}
>
  {#if recording}
    <span class="prompt">Press the new shortcut… <small>Esc to cancel</small></span>
  {:else}
    <Keycaps {accelerator} />
    <span class="change">Change</span>
  {/if}
</button>

<style>
  .shortcut {
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
    min-height: 50px;
    padding: 8px 16px;
    border: 1.5px solid rgb(51 36 26 / 45%);
    border-radius: 10px;
    background: rgb(255 250 235 / 50%);
    text-align: left;
  }

  .shortcut:hover {
    border-color: var(--ink);
  }

  .recording,
  .recording:hover {
    border-style: dashed;
    border-color: var(--teal-ink);
    background: var(--parchment-light);
  }

  .change {
    margin-left: auto;
    color: var(--teal-ink);
    font-weight: 800;
  }

  .prompt {
    color: var(--teal-ink);
    font-family: var(--font-print);
    font-size: 16px;
    font-style: italic;
  }

  small {
    margin-left: 6px;
    color: var(--ink-soft);
    font-family: var(--font-body);
    font-size: 12px;
    font-style: normal;
  }
</style>
