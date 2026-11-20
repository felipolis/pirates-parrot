<!--
  Layers of drifting waves along the bottom edge of a scene. The nearest layer
  is painted in the color of what comes below, so the scene flows into it.
  The same scene is drawn by the website (`site/src/components/Waves.tsx`).
-->
<script lang="ts" module>
  const TILE_WIDTH = 2880;
  const SWELLS_PER_TILE = 6;

  /**
   * A row of smooth swells across a tile twice as wide as the scene. Every
   * swell starts and ends at mid-height with the same slope, so sliding the
   * tile by half its width loops without a seam.
   */
  function swellPath(): string {
    const width = TILE_WIDTH / SWELLS_PER_TILE;
    const curves = Array.from({ length: SWELLS_PER_TILE }, (_, index) => {
      const start = index * width;
      return `C${start + width / 4} 0 ${start + (width * 3) / 4} 100 ${start + width} 50`;
    });
    return `M0 50 ${curves.join(' ')} V100 H0 Z`;
  }

  const SWELL = swellPath();
  const LAYERS = ['far', 'middle', 'near'] as const;
</script>

<div class="waves" aria-hidden="true">
  {#each LAYERS as layer (layer)}
    <svg class="layer {layer}" viewBox="0 0 {TILE_WIDTH} 100" preserveAspectRatio="none">
      <path d={SWELL} />
    </svg>
  {/each}
</div>

<style>
  .waves {
    position: absolute;
    inset: auto 0 0;
    height: 64px;
    overflow: hidden;
    pointer-events: none;
  }

  /* Each tile is twice the width of the scene and slides by half of itself. */
  .layer {
    position: absolute;
    /* Overlaps the edge so no hairline of the scene shows beneath. */
    bottom: -1px;
    left: 0;
    width: 200%;
    animation: drift linear infinite;
  }

  .far {
    height: 100%;
    fill: var(--sea-glow);
    opacity: 0.55;
    animation-duration: 30s;
    animation-direction: reverse;
  }

  .middle {
    height: 70%;
    fill: var(--sea);
    animation-duration: 21s;
    /* Out of step with the other layers, so the crests never line up. */
    animation-delay: -8s;
  }

  .near {
    height: 42%;
    fill: var(--sea-deep);
    animation-duration: 14s;
  }

  @keyframes drift {
    to {
      transform: translateX(-50%);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .layer {
      animation: none;
    }
  }
</style>
