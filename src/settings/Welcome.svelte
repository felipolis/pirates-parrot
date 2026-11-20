<!--
  First-run guide, shown instead of the settings while the speech model is
  missing: get the model, then try a first dictation. It can be skipped at any
  point; the settings hold everything it does.
-->
<script lang="ts">
  import { onMount } from 'svelte';
  import type { EngineStatus, ModelId, ModelInfo } from '../lib/api/types';
  import { onEvent } from '../lib/api/events';
  import Button from '../lib/ui/Button.svelte';
  import Keycaps from '../lib/ui/Keycaps.svelte';
  import ModelSection from './ModelSection.svelte';

  interface Props {
    engine: EngineStatus;
    models: ModelInfo[];
    activeModel: ModelId;
    shortcut: string;
    onSelectModel: (model: ModelId) => void;
    onModelRemoved: () => void;
    onError: (error: unknown) => void;
    /** Leaves the guide for the settings. */
    onDone: () => void;
  }

  let {
    engine,
    models,
    activeModel,
    shortcut,
    onSelectModel,
    onModelRemoved,
    onError,
    onDone,
  }: Props = $props();

  /** What the first dictation produced, once there was one. */
  let firstWords = $state<string | null>(null);

  const ready = $derived(engine.state === 'ready');

  onMount(() => {
    const subscription = onEvent('dictation-result', (result) => {
      if (result.kind === 'delivered') firstWords = result.text;
    }).catch((error: unknown) => {
      onError(error);
      return null;
    });
    return () => void subscription.then((unlisten) => unlisten?.());
  });
</script>

<div class="welcome">
  <header>
    <h2>Welcome aboard!</h2>
    <p>
      Your parrot turns your voice into text, right here on this computer. Two quick steps and
      you’re sailing.
    </p>
  </header>

  <ol class="steps" aria-label="Getting started">
    <li class:current={!ready} class:done={ready}>Give it ears</li>
    <li class:current={ready}>Say hello</li>
  </ol>

  {#if !ready}
    <ModelSection
      {engine}
      {models}
      {activeModel}
      {shortcut}
      onSelect={onSelectModel}
      onRemoved={onModelRemoved}
    />
  {:else if firstWords === null}
    <div class="try">
      <p class="lead">Your parrot is listening for you.</p>
      <p>
        Click into any text box, in any app, and press <Keycaps accelerator={shortcut} />. Say
        something, then press it again. Your words land right where you clicked.
      </p>
    </div>
  {:else}
    <div class="try success">
      <p class="lead">Aye, it heard you!</p>
      <blockquote>{firstWords}</blockquote>
      <p>That’s all there is to it. The settings let you pick a parrot, a language and more.</p>
    </div>
  {/if}

  <footer>
    {#if firstWords !== null}
      <Button variant="primary" onclick={onDone}>Set sail</Button>
    {:else}
      <Button variant="link" onclick={onDone}>Skip to the settings</Button>
    {/if}
  </footer>
</div>

<style>
  .welcome {
    display: grid;
    gap: 18px;
  }

  h2 {
    margin: 0;
    font-family: var(--font-print);
    font-size: 30px;
    font-weight: 400;
  }

  header p {
    margin: 6px 0 0;
    color: var(--ink-soft);
  }

  /* Two numbered waypoints; the current one is inked in, finished ones ticked. */
  .steps {
    counter-reset: step;
    display: flex;
    gap: 22px;
    margin: 0;
    padding: 0;
    list-style: none;
    color: var(--ink-faint);
    font-family: var(--font-print-caps);
    font-size: 17px;
  }

  .steps li {
    display: inline-flex;
    align-items: center;
    gap: 8px;
  }

  .steps li::before {
    content: counter(step, upper-roman);
    counter-increment: step;
    display: grid;
    place-items: center;
    width: 28px;
    height: 28px;
    border: 1.5px solid currentColor;
    border-radius: 50%;
    font-family: var(--font-print);
    font-size: 14px;
  }

  .current {
    color: var(--ink);
  }

  .current::before {
    border-color: var(--brass-dark);
    background: radial-gradient(circle at 35% 30%, var(--brass-light), var(--brass));
  }

  .done {
    color: var(--kelp);
  }

  .try {
    display: grid;
    gap: 10px;
    padding: 18px 20px;
    border: 1.5px dashed rgb(51 36 26 / 35%);
    border-radius: 10px;
    background: rgb(255 250 235 / 40%);
  }

  .try p {
    margin: 0;
    line-height: 1.9;
  }

  .lead {
    font-family: var(--font-print);
    font-size: 21px;
  }

  .success {
    border-style: solid;
    border-color: var(--kelp);
  }

  .success .lead {
    color: var(--kelp);
  }

  blockquote {
    margin: 0;
    padding: 8px 14px;
    border-left: 3px solid var(--brass);
    font-family: var(--font-print);
    font-size: 18px;
    font-style: italic;
  }

  footer {
    display: flex;
    justify-content: flex-end;
  }
</style>
