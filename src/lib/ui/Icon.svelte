<!-- Small ink-drawn icons. They inherit the current text color. -->
<script lang="ts" module>
  const PATHS = {
    chest: [
      'M4 10.5h16v8.5a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1z',
      'M4 10.5V8.5a4.5 4.5 0 0 1 4.5-4.5h7A4.5 4.5 0 0 1 20 8.5v2',
      'M8.5 4.2V20M15.5 4.2V20',
      'M10.8 10.5h2.4v3.2h-2.4z',
    ],
    trumpet: ['M3.5 9.5v5h3l8 5v-15l-8 5z', 'M18 9a4 4 0 0 1 0 6', 'M20.2 6.8a7 7 0 0 1 0 10.4'],
    bell: [
      'M6 16.5v-5a6 6 0 0 1 12 0v5l1.6 2H4.4z',
      'M9.8 20.5a2.3 2.3 0 0 0 4.4 0',
      'M12 5.5V3.5',
    ],
    quill: [
      'M19.5 3.5C12.5 4.5 8 9.5 6.4 18',
      'M19.5 3.5c-.4 5.6-4 10-10.4 11.3',
      'M9.6 11.4l4.2-1M11.9 8l3.4-.6',
      'M6.4 18l-2 2.5',
    ],
    scroll: [
      'M7 4.5h11a2 2 0 0 1 0 4h-1.5V18a2 2 0 0 1-2 2H5.5a2 2 0 0 1 0-4H7z',
      'M7 16V4.5a2 2 0 0 0-2 2v1',
      'M10 9.5h4M10 12.5h4',
    ],
    tricorne: [
      'M2.5 14c3-4.5 6-3.5 7.2-2.2 1-4.5 3.6-4.5 4.6 0 1.2-1.3 4.2-2.3 7.2 2.2-3.8-.7-6.4 1.4-9.5 1.4S6.3 13.3 2.5 14z',
      'M7.5 13.6c3 1 6 1 9 0',
      'M12 8.6v.1',
    ],
    check: ['M5 12.5l4.2 4L19 7'],
  } as const;

  export type IconName = keyof typeof PATHS;
</script>

<script lang="ts">
  interface Props {
    name: IconName;
    size?: number;
  }

  let { name, size = 20 }: Props = $props();
</script>

<svg viewBox="0 0 24 24" width={size} height={size} aria-hidden="true">
  {#each PATHS[name] as path (path)}
    <path d={path} />
  {/each}
</svg>

<style>
  svg {
    flex: none;
    fill: none;
    stroke: currentColor;
    stroke-width: 1.6;
    stroke-linecap: round;
    stroke-linejoin: round;
  }
</style>
