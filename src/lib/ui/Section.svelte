<!--
  One part of a settings sheet: a small-caps heading, a line of explanation
  and its controls. Consecutive sections are divided by a hand-ruled line.
-->
<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    title: string;
    description?: string;
    children: Snippet;
  }

  let { title, description, children }: Props = $props();
</script>

<section class="section">
  <header>
    <h2>{title}</h2>
    {#if description}
      <p>{description}</p>
    {/if}
  </header>
  <div class="body">
    {@render children()}
  </div>
</section>

<style>
  .section:not(:first-child) {
    margin-top: 24px;
    padding-top: 22px;
    border-top: 1px solid rgb(51 36 26 / 22%);
  }

  header {
    margin-bottom: 14px;
  }

  h2 {
    margin: 0;
    font-family: var(--font-print-caps);
    font-size: 20px;
    font-weight: 400;
    line-height: 1.1;
    letter-spacing: 0.01em;
  }

  p {
    margin: 3px 0 0;
    color: var(--ink-soft);
    font-family: var(--font-print);
    font-size: 15px;
    font-style: italic;
  }

  .body {
    display: grid;
    /* Let content shrink to the sheet instead of widening it. */
    grid-template-columns: minmax(0, 1fr);
    gap: 14px;
  }
</style>
