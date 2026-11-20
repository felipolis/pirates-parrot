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

  <!-- A small ship heading for the X; its waterline sits on the route. -->
  <g class="ship" style:transform="translate({ship.x}px, {ship.y}px) scale(0.8)">
    <g class="hull-motion">
      <!-- Bowsprit and the jib, from the masthead to the bowsprit's tip. -->
      <path class="spar" d="M12 -5L20 -9" />
      <path class="sail" d="M1.5 -25.5L19.5 -8.5Q13 -9 9.5 -6.5Q8 -16 1.5 -25.5Z" />
      <!-- Mast, yards and the square sail bellied by the wind. -->
      <path class="spar" d="M0 -4V-28M-8 -24H8M-7 -11H7" />
      <path class="sail" d="M-8 -23.5H8Q11 -17 7 -11.5H-7Q-4 -17 -8 -23.5Z" />
      <path class="seam" d="M-2.6 -23.5Q0 -17 -2.2 -11.5M2.8 -23.5Q5.6 -17 2.4 -11.5" />
      <!-- Pennant streaming ahead with the wind. -->
      <path class="flag" d="M0 -28L8 -26.4L0 -24.8Z" />
      <!-- Hull with a raised stern castle and a rising bow. -->
      <path class="castle" d="M-15 -4V-8H-9V-4Z" />
      <path class="hull" d="M-16 -4H12Q15 -4.5 17 -7Q16 -1 12 2Q9 4 3 4H-11Q-15 4 -16 0Z" />
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
  .spar,
  .seam {
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

  .hull,
  .castle {
    stroke: var(--ink);
    stroke-width: 1;
    stroke-linejoin: round;
  }

  .hull {
    fill: var(--wood);
  }

  .castle {
    fill: var(--wood-light);
  }

  .spar {
    stroke: var(--ink);
    stroke-width: 1.4;
  }

  .sail {
    fill: var(--parchment-light);
    stroke: var(--ink);
    stroke-width: 0.9;
    stroke-linejoin: round;
  }

  .seam {
    stroke: var(--ink);
    stroke-width: 0.5;
    opacity: 0.45;
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
