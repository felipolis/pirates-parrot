<!--
  The settings window: the captain's log, one tab per subject. While the speech
  model is missing, a short guide takes its place. Every change is saved
  immediately.
-->
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
  import {
    faAnchor,
    faCopy,
    faCrow,
    faKeyboard,
    faMicrophoneLines,
    faPaste,
    type IconDefinition,
  } from '@fortawesome/free-solid-svg-icons';
  import { skinById } from '../lib/parrot/skins';
  import OptionCard from '../lib/ui/OptionCard.svelte';
  import Section from '../lib/ui/Section.svelte';
  import Tabs from '../lib/ui/Tabs.svelte';
  import Hero from './Hero.svelte';
  import ModelSection from './ModelSection.svelte';
  import ShortcutField from './ShortcutField.svelte';
  import SkinPicker from './SkinPicker.svelte';
  import Welcome from './Welcome.svelte';

  interface Choice<T> {
    value: T;
    title: string;
    description: string;
    icon?: IconDefinition;
  }

  const TABS = [
    { id: 'model', label: 'Model', icon: faAnchor },
    { id: 'listening', label: 'Listening', icon: faMicrophoneLines },
    { id: 'shortcut', label: 'Shortcut', icon: faKeyboard },
    { id: 'crew', label: 'Crew', icon: faCrow },
  ] as const;

  type TabId = (typeof TABS)[number]['id'];

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
      icon: faPaste,
    },
    {
      value: 'copy',
      title: 'Copy to clipboard',
      description: 'Paste it yourself, wherever you like.',
      icon: faCopy,
    },
  ];

  const app = new AppState();
  let models = $state<ModelInfo[]>([]);
  let languages = $state<LanguageOption[]>([]);
  let microphones = $state<InputDevice[]>([]);
  /** The last thing that went wrong on this page, shown in a banner. */
  let pageError = $state<string | null>(null);
  let tab = $state<TabId>('model');
  /** Whether the first-run guide is shown; decided once the state is loaded. */
  let welcoming = $state(false);

  const skin = $derived(skinById(app.settings?.parrot));
  const status = $derived(engineSummary());
  const engineState = $derived(app.engine.state);

  onMount(() => {
    const connection = app.connect().then(
      (disconnect) => {
        welcoming = app.engine.state === 'notInstalled' || app.engine.state === 'installing';
        return disconnect;
      },
      (error: unknown) => {
        showError(error);
        return null;
      },
    );
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

    {#if welcoming}
      <div class="sheet parchment">
        <Welcome
          engine={app.engine}
          {models}
          activeModel={settings.model}
          shortcut={settings.shortcut}
          onSelectModel={(model) => void save({ model })}
          onModelRemoved={() => void refreshModels()}
          onError={showError}
          onDone={() => (welcoming = false)}
        />
      </div>
    {:else}
      <div class="book">
        <Tabs tabs={TABS} selected={tab} onSelect={(id) => (tab = id)} label="Settings" />
        <div class="sheet parchment" id="{tab}-panel" role="tabpanel" aria-labelledby="{tab}-tab">
          {#if tab === 'model'}
            <Section
              title="Speech model"
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
          {:else if tab === 'listening'}
            <Section title="Language" description="The tongue you speak to your parrot.">
              <label class="field">
                <span class="visually-hidden">Language</span>
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
            </Section>
            <Section title="Microphone" description="The ear your parrot listens with.">
              <label class="field">
                <span class="visually-hidden">Microphone</span>
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
            </Section>
          {:else if tab === 'shortcut'}
            <Section title="Shortcut" description="Ring the bell from any app, any time.">
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
          {:else}
            <Section title="The crew" description="Pick your shipmate for the desktop.">
              <SkinPicker selected={skin.id} onSelect={(parrot) => void save({ parrot })} />
            </Section>
          {/if}
        </div>
      </div>
    {/if}
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
    background: var(--sea-deep);
  }

  .page {
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    gap: 18px;
    max-width: 700px;
    margin: 0 auto;
    padding: 4px 20px 28px;
  }

  /* The tabs stand on the sheet they open, with no gap between them. */
  .book {
    display: grid;
    grid-template-columns: minmax(0, 1fr);
  }

  /* One page of the log, framed by a double ink rule like an old chart. */
  .sheet {
    position: relative;
    padding: 24px 26px 26px;
    border-radius: var(--radius);
    box-shadow: var(--shadow-lift);
  }

  .sheet::before {
    content: '';
    position: absolute;
    inset: 6px;
    border: 1px solid rgb(51 36 26 / 30%);
    border-radius: calc(var(--radius) - 5px);
    box-shadow: inset 0 0 0 3px rgb(51 36 26 / 6%);
    pointer-events: none;
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

  .field {
    display: grid;
    align-content: start;
    gap: 6px;
    /* Long microphone names must not widen the grid column. */
    min-width: 0;
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
