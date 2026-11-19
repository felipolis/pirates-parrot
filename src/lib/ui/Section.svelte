<!-- A chapter of the captain's log: a parchment panel with an ornate heading. -->
<script lang="ts">
  import type { Snippet } from 'svelte';
  import Icon, { type IconName } from './Icon.svelte';

  interface Props {
    title: string;
    icon: IconName;
    description?: string;
    children: Snippet;
  }

  let { title, icon, description, children }: Props = $props();
</script>

<section class="section parchment">
  <header>
    <span class="medallion"><Icon name={icon} size={22} /></span>
    <div>
      <h2>{title}</h2>
      {#if description}
        <p>{description}</p>
      {/if}
    </div>
  </header>
  <div class="flourish" aria-hidden="true"><span></span></div>
  <div class="body">
    {@render children()}
  </div>
</section>

<style>
  .section {
    position: relative;
    padding: 20px 22px 22px;
    border-radius: var(--radius);
    box-shadow: var(--shadow-lift);
  }

  /* Double ink rule, like the frame of an old chart. */
  .section::before {
    content: '';
    position: absolute;
    inset: 6px;
    border: 1px solid rgb(51 36 26 / 30%);
    border-radius: calc(var(--radius) - 5px);
    box-shadow: inset 0 0 0 3px rgb(51 36 26 / 6%);
    pointer-events: none;
  }

  header {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .medallion {
    display: grid;
    place-items: center;
    flex: none;
    width: 44px;
    height: 44px;
    border-radius: 50%;
    background: radial-gradient(
      circle at 35% 30%,
      var(--brass-light),
      var(--brass) 55%,
      var(--brass-dark)
    );
    box-shadow:
      inset 0 0 0 2px rgb(255 240 200 / 35%),
      0 1px 2px rgb(51 36 26 / 40%);
    color: var(--ink);
  }

  h2 {
    margin: 0;
    font-family: var(--font-print-caps);
    font-size: 21px;
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

  /* Hand-ruled divider with a diamond in the middle. */
  .flourish {
    position: relative;
    height: 12px;
    margin: 12px 0 16px;
  }

  .flourish::before {
    content: '';
    position: absolute;
    top: 50%;
    right: 0;
    left: 0;
    border-top: 1px solid rgb(51 36 26 / 35%);
  }

  .flourish span {
    position: absolute;
    top: 50%;
    left: 50%;
    width: 8px;
    height: 8px;
    border: 1px solid rgb(51 36 26 / 55%);
    background: var(--parchment);
    transform: translate(-50%, -50%) rotate(45deg);
  }

  .body {
    display: grid;
    /* Let content shrink to the panel instead of widening it. */
    grid-template-columns: minmax(0, 1fr);
    gap: 16px;
  }
</style>
