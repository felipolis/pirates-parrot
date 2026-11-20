<!--
  Bookmark tabs that sit on top of a parchment sheet. Follows the WAI-ARIA tabs
  pattern: arrow keys, Home and End move between tabs, and only the selected
  tab is in the Tab order. Each tab controls the element with the id
  `<tab id>-panel`, which is labelled by `<tab id>-tab`.
-->
<script lang="ts" generics="T extends string">
  import type { IconDefinition } from '@fortawesome/free-solid-svg-icons';
  import Icon from './Icon.svelte';

  interface Props {
    tabs: readonly { id: T; label: string; icon: IconDefinition }[];
    selected: T;
    onSelect: (id: T) => void;
    label: string;
  }

  let { tabs, selected, onSelect, label }: Props = $props();

  let list = $state<HTMLElement>();

  /** Index of the tab a navigation key moves to, wrapping around the ends. */
  function targetIndex(key: string, current: number): number | null {
    const last = tabs.length - 1;
    switch (key) {
      case 'ArrowRight':
        return current === last ? 0 : current + 1;
      case 'ArrowLeft':
        return current === 0 ? last : current - 1;
      case 'Home':
        return 0;
      case 'End':
        return last;
      default:
        return null;
    }
  }

  function onKeyDown(event: KeyboardEvent, current: number) {
    const index = targetIndex(event.key, current);
    const tab = index === null ? undefined : tabs[index];
    if (!tab) return;
    event.preventDefault();
    onSelect(tab.id);
    list?.querySelector<HTMLElement>(`#${tab.id}-tab`)?.focus();
  }
</script>

<div class="tabs" role="tablist" aria-label={label} bind:this={list}>
  {#each tabs as tab, index (tab.id)}
    {@const active = tab.id === selected}
    <button
      id="{tab.id}-tab"
      class="tab"
      class:active
      role="tab"
      aria-selected={active}
      aria-controls="{tab.id}-panel"
      tabindex={active ? 0 : -1}
      onclick={() => onSelect(tab.id)}
      onkeydown={(event) => onKeyDown(event, index)}
    >
      <Icon icon={tab.icon} size={15} />
      {tab.label}
    </button>
  {/each}
</div>

<style>
  .tabs {
    display: flex;
    justify-content: center;
    gap: 6px;
    padding: 0 12px;
  }

  .tab {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 10px 16px 9px;
    border: 1.5px solid color-mix(in srgb, var(--foam) 22%, transparent);
    border-bottom: none;
    border-radius: 10px 10px 0 0;
    background: color-mix(in srgb, var(--sea-deep) 55%, transparent);
    color: var(--foam-soft);
    font-family: var(--font-print-caps);
    font-size: 16px;
    transition:
      background-color 150ms ease,
      color 150ms ease;
  }

  .tab:hover {
    color: var(--foam);
  }

  /* The open tab is cut from the same parchment as the sheet below it. */
  .active,
  .active:hover {
    border-color: var(--brass);
    background: var(--parchment);
    color: var(--ink);
  }

  .active :global(svg) {
    color: var(--brass-dark);
  }

  @media (prefers-reduced-motion: reduce) {
    .tab {
      transition: none;
    }
  }
</style>
