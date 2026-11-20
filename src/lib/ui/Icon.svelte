<!--
  A Font Awesome Free icon, drawn as inline SVG in the current text color.
  Icons are imported individually from `@fortawesome/free-solid-svg-icons`,
  so only the ones in use are bundled.
-->
<script lang="ts">
  import type { IconDefinition } from '@fortawesome/free-solid-svg-icons';

  interface Props {
    icon: IconDefinition;
    /** Height in pixels; the width follows the icon's proportions. */
    size?: number;
  }

  let { icon, size = 18 }: Props = $props();

  const width = $derived(icon.icon[0]);
  const height = $derived(icon.icon[1]);
  const paths = $derived([icon.icon[4]].flat());
</script>

<svg
  viewBox="0 0 {width} {height}"
  width={(size * width) / height}
  height={size}
  aria-hidden="true"
>
  {#each paths as path (path)}
    <path d={path} />
  {/each}
</svg>

<style>
  svg {
    flex: none;
    fill: currentColor;
  }
</style>
