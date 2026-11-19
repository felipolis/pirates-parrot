<!-- The settings window: the captain's log. Every change is saved immediately. -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { AppState } from '../lib/app-state.svelte';
  import { listLanguages, listMicrophones, listModels, updateSettings } from '../lib/api/commands';
  import { errorMessage } from '../lib/api/errors';
  import type {
    InputDevice,
    LanguageOption,
    ModelInfo,
    OutputMode,
    Settings,
    ShortcutMode,
  } from '../lib/api/types';
  import { skinById } from '../lib/parrot/skins';
  import type { IconName } from '../lib/ui/Icon.svelte';
  import OptionCard from '../lib/ui/OptionCard.svelte';
  import Section from '../lib/ui/Section.svelte';
  import Hero from './Hero.svelte';
  import ModelSection from './ModelSection.svelte';
  import ShortcutField from './ShortcutField.svelte';
  import SkinPicker from './SkinPicker.svelte';

  interface Choice<T> {
    value: T;
    title: string;
    description: string;
    icon?: IconName;
  }

  const SHORTCUT_MODES: Choice<ShortcutMode>[] = [
    {
      value: 'toggle',
      title: 'Press to start, press to stop',
      description: 'Best for longer prompts.',
    },
    {
      value: 'hold',
      title: 'Hold while talking',
      description: 'Walkie-talkie style. Let go to finish.',
    },
  ];

  const OUTPUT_MODES: Choice<OutputMode>[] = [
    {
      value: 'copyAndPaste',
      title: 'Paste where I’m typing',
      description: 'Lands right in your terminal or chat box. Also copied.',
      icon: 'quill',
    },
    {
      value: 'copy',
      title: 'Copy to clipboard',
      description: 'Paste it yourself, wherever you like.',
      icon: 'scroll',
    },
  ];

  const app = new AppState();
  let models = $state<ModelInfo[]>([]);
  let languages = $state<LanguageOption[]>([]);
  let microphones = $state<InputDevice[]>([]);
  /** The last thing that went wrong on this page, shown in a banner. */
  let pageError = $state<string | null>(null);

  const skin = $derived(skinById(app.settings?.parrot));
  const status = $derived(engineSummary());
  const engineState = $derived(app.engine.state);

  onMount(() => {
    const connection = app.connect().catch((error: unknown) => {
      showError(error);
      return null;
    });
    listLanguages().then((list) => (languages = list), showError);
    void refreshMicrophones();

    // Microphones come and go; refresh whenever the window regains focus.
    const onFocus = () => void refreshMicrophones();
    window.addEventListener('focus', onFocus);
    return () => {
      window.removeEventListener('focus', onFocus);
      void connection.then((disconnect) => disconnect?.());
    };
  });

  // Which models are downloaded changes with the engine status. Depending on
  // the state alone skips the many progress updates of a download.
  $effect(() => {
    void engineState;
    void refreshModels();
  });

  function showError(error: unknown) {
    pageError = errorMessage(error);
  }

  async function refreshModels() {
    try {
      models = await listModels();
    } catch (error) {
      showError(error);
    }
  }

  async function refreshMicrophones() {
    try {
      microphones = await listMicrophones();
    } catch (error) {
      showError(error);
    }
  }

  /** Saves a change and reports whether it was accepted. Errors are shown in a banner. */
  async function save(change: Partial<Settings>): Promise<boolean> {
    if (!app.settings) return false;
    try {
      app.settings = await updateSettings({ ...app.settings, ...change });
      pageError = null;
      return true;
    } catch (error) {
      showError(error);
      return false;
    }
  }

  /**
   * Saves a dropdown's new value. If the change is refused the dropdown is put
   * back, since the settings it displays did not change.
   */
  async function saveSelection(
    select: HTMLSelectElement,
    change: Partial<Settings>,
    current: string,
  ) {
    if (!(await save(change))) select.value = current;
  }

  function engineSummary(): { label: string; tone: 'good' | 'pending' | 'bad' } {
    switch (app.engine.state) {
      case 'ready':
        return { label: 'Ready to sail', tone: 'good' };
      case 'installing':
        return { label: 'Taking on cargo', tone: 'pending' };
      case 'loading':
        return { label: 'Waking up', tone: 'pending' };
      case 'notInstalled':
        return { label: 'Needs its ears', tone: 'pending' };
      case 'failed':
        return { label: 'Man overboard', tone: 'bad' };
    }
  }
</script>

<Hero
  {skin}
  mood={app.engine.state === 'ready' ? 'idle' : 'sleeping'}
  status={status.label}
  tone={status.tone}
/>

<main class="page">
  {#if pageError}
    <p class="banner" role="alert">{pageError}</p>
  {/if}

  {#if app.settings}
    {@const settings = app.settings}

    <Section
      title="Speech model"
      icon="chest"
      description="The parrot's ears. Runs on this computer, nothing you say ever leaves it."
    >
      <ModelSection
        engine={app.engine}
        {models}
        activeModel={settings.model}
        shortcut={settings.shortcut}
        onSelect={(model) => void save({ model })}
        onRemoved={() => void refreshModels()}
      />
    </Section>

    <Section title="Listening" icon="trumpet" description="Which tongue, and which ear.">
      <div class="fields">
        <label class="field">
          <span class="label">Language</span>
          <select
            value={settings.language}
            onchange={(event) =>
              void saveSelection(
                event.currentTarget,
                { language: event.currentTarget.value },
                settings.language,
              )}
          >
            {#each languages as language (language.code)}
              <option value={language.code}>{language.name}</option>
            {/each}
          </select>
          <small>Automatic detection gives the best punctuation.</small>
        </label>

        <label class="field">
          <span class="label">Microphone</span>
          <select
            value={settings.microphone ?? ''}
            onchange={(event) =>
              void saveSelection(
                event.currentTarget,
                { microphone: event.currentTarget.value || null },
                settings.microphone ?? '',
              )}
          >
            <option value="">System default</option>
            {#each microphones as microphone (microphone.id)}
              <option value={microphone.id}>{microphone.name}</option>
            {/each}
          </select>
        </label>
      </div>
    </Section>

    <Section title="Shortcut" icon="bell" description="Ring the bell from any app, any time.">
      <ShortcutField
        accelerator={settings.shortcut}
        onChange={(shortcut) => save({ shortcut })}
        onError={showError}
      />
      {#if app.shortcutProblem}
        <p class="banner" role="alert">{app.shortcutProblem}. Try another combination.</p>
      {/if}
      <div class="choices" role="radiogroup" aria-label="Shortcut behavior">
        {#each SHORTCUT_MODES as choice (choice.value)}
          <OptionCard
            title={choice.title}
            description={choice.description}
            selected={settings.shortcutMode === choice.value}
            onSelect={() => void save({ shortcutMode: choice.value })}
          />
        {/each}
      </div>
    </Section>

    <Section
      title="When you finish talking"
      icon="quill"
      description="Where the parrot drops your words."
    >
      <div class="choices" role="radiogroup" aria-label="Output">
        {#each OUTPUT_MODES as choice (choice.value)}
          <OptionCard
            title={choice.title}
            description={choice.description}
            icon={choice.icon}
            selected={settings.output === choice.value}
            onSelect={() => void save({ output: choice.value })}
          />
        {/each}
      </div>
    </Section>

    <Section title="The crew" icon="tricorne" description="Pick your shipmate for the desktop.">
      <SkinPicker selected={skin.id} onSelect={(parrot) => void save({ parrot })} />
    </Section>
  {/if}

  <footer>
    <svg class="compass" viewBox="0 0 40 40" aria-hidden="true">
      <circle cx="20" cy="20" r="17" />
      <path d="M20 4l3.5 16L20 36l-3.5-16z" />
      <path d="M4 20l16-3.5L36 20l-16 3.5z" />
    </svg>
    <p>
      Speech recognition by NVIDIA Nemotron 3.5 ASR Streaming,<br />
      running on your own ship with sherpa-onnx.
    </p>
  </footer>
</main>

<style>
  :global(html) {
    background: var(--sea-deep);
  }

  :global(body) {
    min-height: 100vh;
    background:
      radial-gradient(ellipse 80% 260px at 50% 0%, var(--sea-glow), transparent),
      linear-gradient(var(--sea) 0, var(--sea-deep) 520px);
    background-attachment: fixed;
  }

  .page {
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    gap: 18px;
    max-width: 700px;
    margin: 0 auto;
    padding: 6px 20px 28px;
  }

  .banner {
    margin: 0;
    padding: 10px 14px;
    border-left: 4px solid var(--scarlet);
    border-radius: 6px;
    background: var(--scarlet-wash);
    color: var(--scarlet-dark);
    font-weight: 700;
  }

  .fields {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 16px;
  }

  .field {
    display: grid;
    align-content: start;
    gap: 6px;
    /* Long microphone names must not widen the grid column. */
    min-width: 0;
  }

  .label {
    font-family: var(--font-print-caps);
    font-size: 17px;
  }

  .field small {
    color: var(--ink-soft);
    font-family: var(--font-print);
    font-size: 14px;
    font-style: italic;
  }

  select {
    width: 100%;
    height: 40px;
    text-overflow: ellipsis;
    padding: 0 36px 0 12px;
    border: 1.5px solid rgb(51 36 26 / 45%);
    border-radius: 10px;
    background:
      url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 12 8'%3E%3Cpath d='M1 1.5l5 5 5-5' fill='none' stroke='%2333241a' stroke-width='1.6' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E")
        no-repeat right 14px center / 12px 8px,
      rgb(255 250 235 / 55%);
    font-weight: 700;
    appearance: none;
  }

  select:hover {
    border-color: var(--ink);
  }

  .choices {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 10px;
  }

  footer {
    display: grid;
    justify-items: center;
    gap: 6px;
    padding-top: 6px;
    color: var(--foam-soft);
    text-align: center;
  }

  footer p {
    margin: 0;
    font-family: var(--font-print);
    font-size: 14px;
    font-style: italic;
    opacity: 0.85;
  }

  .compass {
    width: 34px;
    height: 34px;
    fill: none;
    stroke: var(--brass-light);
    stroke-width: 1.2;
    stroke-linejoin: round;
  }

  .compass path:first-of-type {
    fill: var(--brass);
  }
</style>
