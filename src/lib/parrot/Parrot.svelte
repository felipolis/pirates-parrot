<!--
  The Pirate's Parrot mascot. One layered SVG drawing is recolored by the skin
  palette and dressed with the skin's accessories; CSS animations bring each
  mood to life. While listening, the beak follows the microphone level.
-->
<script lang="ts">
  import type { Mood } from './mood';
  import type { ParrotSkin } from './skins';

  interface Props {
    skin: ParrotSkin;
    mood?: Mood;
    /** Microphone loudness in 0..1, used while listening. */
    level?: number;
    size?: number;
  }

  let { skin, mood = 'idle', level = 0, size = 80 }: Props = $props();

  const palette = $derived(skin.palette);
  const wears = $derived(new Set(skin.accessories));
  const showsCrest = $derived(!wears.has('tricorne') && !wears.has('bandana'));
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  class="parrot mood-{mood}"
  class:ghostly={skin.ghostly}
  viewBox="0 0 100 100"
  width={size}
  height={size}
  style:--level={level}
  role="img"
  aria-label={skin.name}
>
  <g class="figure">
    <g class="tail">
      <path d="M46 78C42 88 40 94 41 99C45 96 48 90 50 80Z" fill={palette.tail} />
      <path d="M54 78C58 88 60 94 59 99C55 96 52 90 50 80Z" fill={palette.tail} />
      <path d="M47.5 78C47.5 88 48.5 94 50 100C51.5 94 52.5 88 52.5 78Z" fill={palette.tailTip} />
    </g>

    <g class="wing wing-left">
      <ellipse cx="29" cy="68" rx="8" ry="17" transform="rotate(14 29 68)" fill={palette.wing} />
      <ellipse
        cx="26.5"
        cy="79"
        rx="5.5"
        ry="7"
        transform="rotate(14 26.5 79)"
        fill={palette.wingTip}
      />
    </g>
    <g class="wing wing-right">
      <ellipse cx="71" cy="68" rx="8" ry="17" transform="rotate(-14 71 68)" fill={palette.wing} />
      <ellipse
        cx="73.5"
        cy="79"
        rx="5.5"
        ry="7"
        transform="rotate(-14 73.5 79)"
        fill={palette.wingTip}
      />
    </g>

    <ellipse cx="50" cy="67" rx="21" ry="23" fill={palette.body} />
    <ellipse cx="50" cy="72" rx="13.5" ry="15" fill={palette.belly} />

    <g fill={palette.feet}>
      <circle cx="41.5" cy="90.5" r="2.4" />
      <circle cx="45" cy="91.5" r="2.4" />
      <circle cx="55" cy="91.5" r="2.4" />
      <circle cx="58.5" cy="90.5" r="2.4" />
    </g>

    <g class="head">
      {#if showsCrest}
        <g fill={palette.crest}>
          <path d="M50 20C46 10 40 6 36 7C40 11 44 16 46 22Z" />
          <path d="M50 19C50 9 52 4 55 2C55 8 54 14 53 21Z" />
          <path d="M52 20C57 12 62 9 66 10C61 13 57 18 55 23Z" />
        </g>
      {/if}

      <circle cx="50" cy="40" r="22" fill={palette.body} />
      <ellipse cx="40.5" cy="40" rx="8.5" ry="9.5" fill={palette.face} />
      <ellipse cx="59.5" cy="40" rx="8.5" ry="9.5" fill={palette.face} />
      <ellipse cx="35" cy="49" rx="3.6" ry="2.4" fill={palette.cheek} opacity="0.8" />
      <ellipse cx="65" cy="49" rx="3.6" ry="2.4" fill={palette.cheek} opacity="0.8" />

      <g class="eyes">
        <g class="pupils">
          <circle cx="41" cy="39" r="4.4" fill={palette.eye} />
          <circle cx="59" cy="39" r="4.4" fill={palette.eye} />
          <circle cx="42.6" cy="37.4" r="1.5" fill="#fff" />
          <circle cx="60.6" cy="37.4" r="1.5" fill="#fff" />
        </g>
      </g>

      <ellipse cx="50" cy="53" rx="4" ry="3.2" fill="#5a2a2a" />
      <path
        class="jaw"
        d="M45.5 50C47 53 53 53 54.5 50C54.5 54.5 52.5 57.5 50 58.5C47.5 57.5 45.5 54.5 45.5 50Z"
        fill={palette.beakShade}
      />
      <path
        d="M43.5 44C44 40.5 56 40.5 56.5 44C57 50 53.5 55 50 57C46.5 55 43 50 43.5 44Z"
        fill={palette.beak}
      />

      {#if wears.has('bandana')}
        <g fill={palette.bandana}>
          <path d="M28.5 34C29 18 71 18 71.5 34C62 29.5 38 29.5 28.5 34Z" />
          <path d="M70 30l9-5-1.5 8Z" />
          <path d="M70 31.5l8.5 6-6.5 1.5Z" />
        </g>
        <g fill="#fff" opacity="0.85">
          <circle cx="40" cy="25" r="1.3" />
          <circle cx="50" cy="23" r="1.3" />
          <circle cx="60" cy="25" r="1.3" />
          <circle cx="45" cy="29" r="1.1" />
          <circle cx="55" cy="29" r="1.1" />
        </g>
      {/if}

      {#if wears.has('tricorne')}
        <path
          d="M13 30C22 20 33 22 37 24C39 11 61 11 63 24C67 22 78 20 87 30C75 27.5 63 33 50 33C37 33 25 27.5 13 30Z"
          fill={palette.hat}
        />
        <path
          d="M13 30C25 27.5 37 33 50 33C63 33 75 27.5 87 30"
          fill="none"
          stroke={palette.hatTrim}
          stroke-width="2"
          stroke-linecap="round"
        />
        <circle cx="50" cy="22.5" r="3.3" fill="#f6f1e7" />
        <rect x="48.6" y="24.6" width="2.8" height="1.8" rx="0.6" fill="#f6f1e7" />
        <circle cx="48.8" cy="22.2" r="0.75" fill={palette.hat} />
        <circle cx="51.2" cy="22.2" r="0.75" fill={palette.hat} />
      {/if}

      {#if wears.has('eyepatch')}
        <g fill="none" stroke="#1f1b1a" stroke-width="1.6" stroke-linecap="round">
          <path d="M29 31C40 27 50 30 57 35" />
          <path d="M64.5 41C68 43 70 45 71.5 47" />
        </g>
        <ellipse cx="59.5" cy="39.5" rx="6.2" ry="5.6" fill="#1f1b1a" />
      {/if}

      {#if wears.has('earring')}
        <circle cx="29.5" cy="47.5" r="3" fill="none" stroke="#e2b13c" stroke-width="1.6" />
      {/if}
    </g>
  </g>
</svg>

<style>
  .parrot {
    display: block;
    overflow: visible;
  }

  .parrot :global(*) {
    transform-box: view-box;
  }

  .ghostly {
    opacity: 0.82;
  }

  .figure {
    transform-origin: 50px 92px;
    animation: breathe 3.2s ease-in-out infinite;
  }

  .ghostly .figure {
    animation: haunt 3.6s ease-in-out infinite;
  }

  .head {
    transform-origin: 50px 58px;
    transition: transform 240ms ease;
  }

  .eyes {
    transform-origin: 50px 39px;
    animation: blink 4.6s infinite;
  }

  .pupils {
    transition: transform 240ms ease;
  }

  .jaw {
    transition: transform 70ms linear;
  }

  .wing-left {
    transform-origin: 33px 56px;
  }

  .wing-right {
    transform-origin: 67px 56px;
  }

  /* Listening: lean in and talk along with the user. */
  .mood-listening .head {
    animation: tilt 2.4s ease-in-out infinite;
  }

  .mood-listening .jaw {
    transform: translateY(calc(var(--level) * 4px));
  }

  /* Thinking: eyes up while the last words are transcribed. */
  .mood-thinking .pupils {
    transform: translate(1px, -1.6px);
  }

  .mood-thinking .head {
    transform: rotate(-6deg);
  }

  /* Happy: a hop and a flap. */
  .mood-happy .figure {
    animation: hop 0.5s ease-out 2;
  }

  .mood-happy .wing-left {
    animation: flap-left 0.25s ease-in-out 4;
  }

  .mood-happy .wing-right {
    animation: flap-right 0.25s ease-in-out 4;
  }

  /* Error: a quick shake. */
  .mood-error .figure {
    animation: shake 0.4s ease-in-out 2;
  }

  /* Sleeping: eyes closed, slow breathing. */
  .mood-sleeping .eyes {
    animation: none;
    transform: scaleY(0.12);
  }

  .mood-sleeping .figure {
    animation-duration: 5s;
  }

  @keyframes breathe {
    0%,
    100% {
      transform: translateY(0) scale(1);
    }
    50% {
      transform: translateY(-1.2px) scale(1.012);
    }
  }

  @keyframes haunt {
    0%,
    100% {
      transform: translateY(0);
    }
    50% {
      transform: translateY(-4px);
    }
  }

  @keyframes blink {
    0%,
    95%,
    100% {
      transform: scaleY(1);
    }
    97.5% {
      transform: scaleY(0.1);
    }
  }

  @keyframes tilt {
    0%,
    100% {
      transform: rotate(-4deg);
    }
    50% {
      transform: rotate(4deg);
    }
  }

  @keyframes hop {
    0%,
    100% {
      transform: translateY(0);
    }
    40% {
      transform: translateY(-6px);
    }
  }

  @keyframes flap-left {
    50% {
      transform: rotate(28deg);
    }
  }

  @keyframes flap-right {
    50% {
      transform: rotate(-28deg);
    }
  }

  @keyframes shake {
    25% {
      transform: translateX(-2.5px) rotate(-3deg);
    }
    75% {
      transform: translateX(2.5px) rotate(3deg);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .parrot :global(*) {
      animation: none !important;
      transition: none !important;
    }
  }
</style>
