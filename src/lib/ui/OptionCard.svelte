<!--
  A selectable card that behaves like a radio button; the chosen one is marked
  with a wax seal. Place inside an element with role="radiogroup".
-->
<script lang="ts">
  import { faCheck, type IconDefinition } from '@fortawesome/free-solid-svg-icons';
  import type { Snippet } from 'svelte';
  import Icon from './Icon.svelte';

  interface Props {
    title: string;
    description?: string;
    icon?: IconDefinition;
    selected: boolean;
    onSelect: () => void;
    /** Content under the card, such as a badge or a secondary action. */
    extra?: Snippet;
  }

  let { title, description, icon, selected, onSelect, extra }: Props = $props();
</script>

<div class="option" class:selected>
  <button class="choose" role="radio" aria-checked={selected} onclick={onSelect}>
    <span class="seal" aria-hidden="true">
      {#if selected}<Icon icon={faCheck} size={11} />{/if}
    </span>
    {#if icon}
      <span class="icon"><Icon {icon} size={20} /></span>
    {/if}
    <span class="title">{title}</span>
    {#if description}
      <span class="description">{description}</span>
    {/if}
  </button>
  {#if extra}
    <div class="extra">{@render extra()}</div>
  {/if}
</div>

<style>
  .option {
    position: relative;
    display: flex;
    flex-direction: column;
    border: 1.5px solid rgb(51 36 26 / 28%);
    border-radius: 10px;
    background: rgb(255 250 235 / 45%);
    transition:
      border-color 120ms ease,
      background-color 120ms ease,
      transform 120ms ease;
  }

  .option:hover {
    border-color: rgb(51 36 26 / 55%);
    background: rgb(255 250 235 / 70%);
  }

  .selected,
  .selected:hover {
    border-color: var(--ink);
    background: var(--parchment-light);
    box-shadow:
      inset 0 0 0 1px var(--ink),
      0 3px 8px rgb(51 36 26 / 18%);
  }

  .choose {
    display: grid;
    flex: 1;
    align-content: start;
    gap: 3px;
    padding: 14px 40px 14px 16px;
    border: none;
    border-radius: inherit;
    background: none;
    text-align: left;
  }

  .seal {
    position: absolute;
    top: 12px;
    right: 12px;
    display: grid;
    place-items: center;
    width: 22px;
    height: 22px;
    border: 1.5px solid rgb(51 36 26 / 45%);
    border-radius: 50%;
    color: var(--wax-text);
  }

  /* A slightly irregular blob of red wax. */
  .selected .seal {
    border: none;
    border-radius: 48% 52% 45% 55% / 55% 45% 55% 45%;
    background: radial-gradient(
      circle at 35% 30%,
      var(--scarlet-light),
      var(--scarlet) 55%,
      var(--scarlet-dark)
    );
    box-shadow:
      0 0 0 3px rgb(181 53 42 / 20%),
      0 1px 2px rgb(51 36 26 / 45%);
    transform: rotate(-8deg) scale(1.08);
  }

  .icon {
    margin-bottom: 4px;
    color: var(--teal-ink);
  }

  .title {
    font-family: var(--font-print-caps);
    font-size: 18px;
    line-height: 1.15;
  }

  .description {
    color: var(--ink-soft);
    font-size: 13px;
  }

  .extra {
    padding: 0 16px 14px;
  }
</style>
