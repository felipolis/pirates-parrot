<!-- The crew: a gallery of framed portraits, one per parrot skin. -->
<script lang="ts">
  import Parrot from '../lib/parrot/Parrot.svelte';
  import { SKINS } from '../lib/parrot/skins';

  interface Props {
    selected: string;
    onSelect: (skin: string) => void;
  }

  let { selected, onSelect }: Props = $props();

  let hovered = $state<string | null>(null);
</script>

<div class="crew" role="radiogroup" aria-label="Parrot">
  {#each SKINS as skin (skin.id)}
    {@const aboard = skin.id === selected}
    <button
      class="member"
      class:aboard
      role="radio"
      aria-checked={aboard}
      onclick={() => onSelect(skin.id)}
      onpointerenter={() => (hovered = skin.id)}
      onpointerleave={() => (hovered = null)}
    >
      <span class="frame">
        <Parrot {skin} mood={hovered === skin.id ? 'listening' : 'idle'} size={86} />
      </span>
      <span class="plate">{skin.name}</span>
      <span class="tagline">{skin.tagline}</span>
      {#if aboard}
        <span class="ribbon">On deck</span>
      {/if}
    </button>
  {/each}
</div>

<style>
  .crew {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
    gap: 14px 12px;
  }

  .member {
    position: relative;
    display: grid;
    justify-items: center;
    align-content: start;
    gap: 6px;
    padding: 4px 4px 8px;
    border: none;
    background: none;
    text-align: center;
  }

  /* Arched portrait in a gilded frame, with a slice of sea behind the parrot. */
  .frame {
    display: grid;
    place-items: center;
    width: 118px;
    height: 132px;
    padding-top: 6px;
    border: 5px solid var(--brass);
    border-radius: 60px 60px 12px 12px;
    background:
      radial-gradient(circle at 50% 25%, rgb(255 255 255 / 18%), transparent 60%),
      linear-gradient(var(--sea-glow), var(--sea) 70%, var(--sea-deep));
    box-shadow:
      inset 0 0 0 2px var(--brass-dark),
      inset 0 6px 14px rgb(0 0 0 / 35%),
      0 0 0 1.5px var(--brass-dark),
      0 4px 10px rgb(51 36 26 / 30%);
    transition: transform 150ms ease;
  }

  .member:hover .frame {
    transform: translateY(-3px) rotate(-1.5deg);
  }

  .aboard .frame {
    border-color: var(--brass-light);
    box-shadow:
      inset 0 0 0 2px var(--brass-dark),
      inset 0 6px 14px rgb(0 0 0 / 35%),
      0 0 0 1.5px var(--brass-dark),
      0 0 0 6px rgb(196 145 47 / 35%),
      0 6px 14px rgb(51 36 26 / 35%);
  }

  .plate {
    margin-top: 4px;
    padding: 2px 12px 3px;
    border: 1px solid var(--brass-dark);
    border-radius: 4px;
    background: linear-gradient(var(--brass-light), var(--brass));
    color: var(--ink);
    font-family: var(--font-print-caps);
    font-size: 15px;
    line-height: 1.2;
  }

  .tagline {
    padding: 0 6px;
    color: var(--ink-soft);
    font-family: var(--font-print);
    font-size: 13.5px;
    font-style: italic;
    line-height: 1.3;
  }

  .ribbon {
    position: absolute;
    top: 0;
    right: 6px;
    padding: 3px 10px 4px;
    border-radius: 3px;
    background: var(--scarlet);
    box-shadow: 0 2px 4px rgb(51 36 26 / 35%);
    color: var(--wax-text);
    font-family: var(--font-print-caps);
    font-size: 13px;
    transform: rotate(8deg);
  }

  @media (prefers-reduced-motion: reduce) {
    .frame {
      transition: none;
    }
  }
</style>
