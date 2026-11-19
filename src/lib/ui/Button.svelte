<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    variant?: 'primary' | 'secondary' | 'link';
    onclick: () => void;
    children: Snippet;
  }

  let { variant = 'secondary', onclick, children }: Props = $props();
</script>

<button class={variant} {onclick}>{@render children()}</button>

<style>
  button {
    padding: 8px 16px 9px;
    border: 1.5px solid var(--ink);
    border-radius: 999px;
    font-weight: 800;
    white-space: nowrap;
    transition:
      transform 100ms ease,
      background-color 120ms ease;
  }

  button:active {
    transform: translateY(1px);
  }

  .primary {
    background: var(--teal-ink);
    box-shadow: 0 2px 0 var(--ink);
    color: var(--parchment-light);
  }

  .primary:hover {
    background: var(--teal-ink-dark);
  }

  .secondary {
    background: rgb(255 250 235 / 60%);
    color: var(--ink);
  }

  .secondary:hover {
    background: var(--parchment-light);
  }

  .link {
    padding: 0;
    border: none;
    background: none;
    color: var(--scarlet);
    font-size: 12px;
    text-decoration: underline;
  }
</style>
