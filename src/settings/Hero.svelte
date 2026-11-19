<!--
  Banner of the settings window: the parrot perched on the ship's rail over a
  moving sea, with the app's name and a pennant showing the engine status.
-->
<script lang="ts">
  import type { Mood } from '../lib/parrot/mood';
  import Parrot from '../lib/parrot/Parrot.svelte';
  import type { ParrotSkin } from '../lib/parrot/skins';

  interface Props {
    skin: ParrotSkin;
    mood: Mood;
    status: string;
    tone: 'good' | 'pending' | 'bad';
  }

  let { skin, mood, status, tone }: Props = $props();
</script>

<header class="hero">
  <div class="stars" aria-hidden="true"></div>
  <div class="moon" aria-hidden="true"></div>
  <div class="wave far" aria-hidden="true"></div>
  <div class="wave near" aria-hidden="true"></div>

  <div class="rail" aria-hidden="true"></div>
  <div class="parrot">
    <Parrot {skin} {mood} size={118} />
  </div>

  <div class="titles">
    <h1>Pirate's Parrot</h1>
    <p>Speak your mind, matey. I'll write it down.</p>
    <span class="pennant {tone}">{status}</span>
  </div>
</header>

<style>
  .hero {
    --wave: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='120' height='28'%3E%3Cpath d='M0 14 Q15 4 30 14 T60 14 T90 14 T120 14 V28 H0Z'/%3E%3C/svg%3E");

    position: relative;
    height: 200px;
    overflow: hidden;
  }

  .stars {
    position: absolute;
    inset: 0 0 60px;
    background-image:
      radial-gradient(1.5px 1.5px at 12% 22%, var(--foam), transparent),
      radial-gradient(1px 1px at 28% 12%, var(--foam), transparent),
      radial-gradient(1.5px 1.5px at 44% 30%, var(--foam), transparent),
      radial-gradient(1px 1px at 58% 10%, var(--foam), transparent),
      radial-gradient(1.5px 1.5px at 71% 26%, var(--foam), transparent),
      radial-gradient(1px 1px at 92% 40%, var(--foam), transparent),
      radial-gradient(1px 1px at 36% 48%, var(--foam), transparent),
      radial-gradient(1.5px 1.5px at 66% 52%, var(--foam), transparent);
    opacity: 0;
  }

  .moon {
    position: absolute;
    top: 26px;
    right: 44px;
    width: 46px;
    height: 46px;
    border-radius: 50%;
    background: radial-gradient(circle at 38% 35%, #fff, var(--sky-light) 60%);
    box-shadow: 0 0 40px 10px rgb(247 227 168 / 35%);
  }

  .wave {
    position: absolute;
    right: 0;
    left: 0;
    -webkit-mask: var(--wave) repeat-x;
    mask: var(--wave) repeat-x;
    animation: drift 14s linear infinite;
  }

  .far {
    bottom: 26px;
    height: 28px;
    background: var(--sea-glow);
    opacity: 0.45;
    -webkit-mask-size: 160px 28px;
    mask-size: 160px 28px;
    animation-duration: 22s;
  }

  .near {
    bottom: 0;
    height: 40px;
    background: var(--sea-deep);
    -webkit-mask-size: 160px 40px;
    mask-size: 160px 40px;
    animation-direction: reverse;
  }

  /* The ship's rail the parrot perches on: a plank over turned posts. */
  .rail {
    position: absolute;
    bottom: 0;
    left: 18px;
    width: 168px;
    height: 46px;
    border-top: 13px solid var(--wood-light);
    border-radius: 3px 3px 0 0;
    background: repeating-linear-gradient(
      90deg,
      transparent 0 14px,
      var(--wood-dark) 14px 24px,
      transparent 24px 34px
    );
    box-shadow: inset 0 1px 0 rgb(255 220 170 / 25%);
  }

  .parrot {
    position: absolute;
    bottom: 36px;
    left: 44px;
  }

  .titles {
    position: absolute;
    top: 50%;
    right: 24px;
    left: 200px;
    transform: translateY(-58%);
  }

  h1 {
    margin: 0;
    color: var(--foam);
    font-family: var(--font-flag);
    font-size: 46px;
    font-weight: 400;
    line-height: 1;
    text-shadow: 0 3px 0 rgb(0 0 0 / 30%);
  }

  p {
    margin: 6px 0 12px;
    color: var(--foam-soft);
    font-family: var(--font-print);
    font-size: 17px;
    font-style: italic;
  }

  /* A swallowtail flag. */
  .pennant {
    display: inline-block;
    padding: 4px 26px 5px 12px;
    clip-path: polygon(0 0, 100% 0, 86% 50%, 100% 100%, 0 100%);
    color: var(--foam);
    font-family: var(--font-print-caps);
    font-size: 15px;
  }

  .good {
    background: var(--kelp);
  }

  .pending {
    background: var(--brass-dark);
  }

  .bad {
    background: var(--scarlet);
  }

  @keyframes drift {
    to {
      -webkit-mask-position: 160px 0;
      mask-position: 160px 0;
    }
  }

  @media (prefers-color-scheme: dark) {
    .stars {
      opacity: 0.8;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .wave {
      animation: none;
    }
  }
</style>
