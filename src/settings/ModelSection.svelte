<!-- Status of the active speech model, its download, and the choice of model. -->
<script lang="ts">
  import { cancelModelInstall, installModel, removeModel } from '../lib/api/commands';
  import { errorMessage } from '../lib/api/errors';
  import type { EngineStatus, ModelId, ModelInfo } from '../lib/api/types';
  import { formatDuration, formatMegabytes } from '../lib/format';
  import Button from '../lib/ui/Button.svelte';
  import Keycaps from '../lib/ui/Keycaps.svelte';
  import OptionCard from '../lib/ui/OptionCard.svelte';
  import VoyageProgress from '../lib/ui/VoyageProgress.svelte';

  interface Props {
    engine: EngineStatus;
    models: ModelInfo[];
    /** The model chosen in the settings. */
    activeModel: ModelId;
    shortcut: string;
    onSelect: (model: ModelId) => void;
    /** Called after a model was deleted so the list can be refreshed. */
    onRemoved: () => void;
  }

  let { engine, models, activeModel, shortcut, onSelect, onRemoved }: Props = $props();

  let actionError = $state<string | null>(null);

  const active = $derived(models.find((model) => model.id === activeModel));

  async function run(action: () => Promise<void>) {
    actionError = null;
    try {
      await action();
    } catch (error) {
      actionError = errorMessage(error);
    }
  }

  function remove(model: ModelId) {
    void run(async () => {
      await removeModel(model);
      onRemoved();
    });
  }
</script>

<div class="status">
  {#if engine.state === 'notInstalled'}
    <p>
      Your parrot needs its ears before the voyage. Download the <strong>{active?.name}</strong>
      model ({formatMegabytes(active?.downloadBytes ?? 0)}) once; it never leaves this computer.
    </p>
    <Button variant="primary" onclick={() => void run(installModel)}>Download model</Button>
  {:else if engine.state === 'installing'}
    {#if engine.progress.stage === 'downloading'}
      {@const { downloaded, total } = engine.progress}
      <VoyageProgress fraction={downloaded / total} label="Downloading the speech model" />
      <p>Sailing in… {formatMegabytes(downloaded)} of {formatMegabytes(total)}</p>
    {:else}
      <VoyageProgress fraction={null} label="Unpacking the speech model" />
      <p>Treasure found! Unpacking it…</p>
    {/if}
    <Button onclick={() => void run(cancelModelInstall)}>Cancel</Button>
  {:else if engine.state === 'loading'}
    <p>The parrot is waking up…</p>
  {:else if engine.state === 'ready'}
    <p>
      <strong>Ready to sail.</strong> Press <Keycaps accelerator={shortcut} /> or click the parrot to
      start talking.
    </p>
  {:else}
    <p class="failed">{engine.message}</p>
    <Button variant="primary" onclick={() => void run(installModel)}>Download again</Button>
  {/if}
</div>

{#if actionError}
  <p class="failed" role="alert">{actionError}</p>
{/if}

<div class="models" role="radiogroup" aria-label="Speech model">
  {#each models as model (model.id)}
    <OptionCard
      title={model.name}
      description="{formatDuration(model.chunkMs)} · {model.description}"
      selected={model.id === activeModel}
      onSelect={() => onSelect(model.id)}
    >
      {#snippet extra()}
        <div class="footer">
          {#if model.installed}
            <span class="aboard">In the hold</span>
            {#if model.id !== activeModel}
              <Button variant="link" onclick={() => remove(model.id)}>Delete</Button>
            {/if}
          {:else}
            <span class="size">{formatMegabytes(model.downloadBytes)} download</span>
          {/if}
        </div>
      {/snippet}
    </OptionCard>
  {/each}
</div>

<style>
  .status {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 12px 16px;
    padding: 14px 16px;
    border: 1.5px dashed rgb(51 36 26 / 35%);
    border-radius: 10px;
    background: rgb(255 250 235 / 40%);
  }

  .status p {
    flex: 1 1 260px;
    margin: 0;
  }

  .failed {
    margin: 0;
    color: var(--scarlet);
    font-weight: 700;
  }

  .models {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 10px;
  }

  .footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }

  .aboard {
    padding: 1px 9px 2px;
    border-radius: 999px;
    background: var(--kelp);
    color: var(--parchment-light);
    font-size: 12px;
    font-weight: 800;
  }

  .size {
    color: var(--ink-faint);
    font-size: 12px;
    font-weight: 700;
  }
</style>
