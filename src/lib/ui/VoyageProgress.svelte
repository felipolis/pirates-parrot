<!--
  Progress drawn as a voyage on a treasure map: a ship sails along a dotted
  route towards the X. With no fraction, it bobs at the X (indeterminate).
-->
<script lang="ts">
  interface Props {
    /** Progress in 0..1, or `null` when the remaining time is unknown. */
    fraction: number | null;
    label: string;
  }

  let { fraction, label }: Props = $props();

  const ROUTE = 'M16 46C66 14 112 62 166 36S264 12 300 32';

  let route = $state<SVGPathElement>();

  const ship = $derived.by(() => {
    if (!route) return { x: 16, y: 46 };
    const point = route.getPointAtLength(route.getTotalLength() * (fraction ?? 1));
    return { x: point.x, y: point.y };
  });
</script>

<svg
  class="voyage"
  class:arrived={fraction === null}
  viewBox="0 0 320 64"
  role="progressbar"
  aria-label={label}
  aria-valuemin={0}
  aria-valuemax={100}
  aria-valuenow={fraction === null ? undefined : Math.round(fraction * 100)}
>
  <path class="route" d={ROUTE} pathLength="1" bind:this={route} />
  <path class="sailed" d={ROUTE} pathLength="1" stroke-dasharray="{fraction ?? 1} 1" />
  <circle class="port" cx="16" cy="46" r="4" />
  <path class="treasure" d="M294 26l12 12M306 26l-12 12" />

  <g class="ship" style:transform="translate({ship.x}px, {ship.y - 4}px)">
    <g class="hull-motion">
      <path class="mast" d="M0 3V-15" />
      <path class="sail" d="M0.5-14Q11-8 0.5-1Z" />
      <path class="sail" d="M-0.5-12Q-8-7-0.5-2Z" />
      <path class="flag" d="M0-15l6 2-6 2Z" />
      <path class="hull" d="M-12 2h24l-4.5 6.5h-15Z" />
    </g>
  </g>
</svg>

<style>
  .voyage {
    display: block;
    width: 100%;
    height: auto;
    overflow: visible;
  }

  .route,
  .sailed,
  .treasure,
  .mast {
    fill: none;
    stroke-linecap: round;
  }

  .route {
    stroke: rgb(51 36 26 / 40%);
    stroke-width: 2;
    stroke-dasharray: 0.012 0.014;
  }

  .sailed {
    stroke: var(--teal-ink);
    stroke-width: 3;
    transition: stroke-dasharray 150ms linear;
  }

  .port {
    fill: var(--teal-ink);
  }

  .treasure {
    stroke: var(--scarlet);
    stroke-width: 3.5;
  }

  .ship {
    transition: transform 150ms linear;
  }

  .hull {
    fill: var(--wood);
    stroke: var(--ink);
    stroke-width: 1;
    stroke-linejoin: round;
  }

  .mast {
    stroke: var(--ink);
    stroke-width: 1.6;
  }

  .sail {
    fill: var(--parchment-light);
    stroke: var(--ink);
    stroke-width: 1;
  }

  .flag {
    fill: var(--scarlet);
  }

  .hull-motion {
    animation: sway 2.2s ease-in-out infinite;
  }

  .arrived .hull-motion {
    animation-duration: 1.1s;
  }

  @keyframes sway {
    0%,
    100% {
      transform: translateY(0) rotate(-3deg);
    }
    50% {
      transform: translateY(-1.5px) rotate(3deg);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .hull-motion,
    .ship,
    .sailed {
      animation: none;
      transition: none;
    }
  }
</style>
