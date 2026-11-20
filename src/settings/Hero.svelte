<!--
  Banner of the settings window: the parrot perched on the ship's rail under a
  starry sky, over a moving sea, with the app's name and a pennant showing the
  engine status.
-->
<script lang="ts">
  import type { Mood } from '../lib/parrot/mood';
  import Parrot from '../lib/parrot/Parrot.svelte';
  import type { ParrotSkin } from '../lib/parrot/skins';
  import Waves from '../lib/ui/Waves.svelte';

  interface Props {
    skin: ParrotSkin;
    mood: Mood;
    status: string;
    tone: 'good' | 'pending' | 'bad';
  }

  let { skin, mood, status, tone }: Props = $props();
</script>

<header class="hero">
  <div class="stars near" aria-hidden="true"></div>
  <div class="stars far" aria-hidden="true"></div>
  <div class="moon" aria-hidden="true"></div>
  <Waves />

  <div class="rail" aria-hidden="true"></div>
  <div class="parrot">
    <Parrot {skin} {mood} size={112} />
  </div>

  <div class="titles">
    <h1>Pirate’s Parrot</h1>
    <p>Speak your mind, matey. I’ll write it down.</p>
    <span class="pennant {tone}">{status}</span>
  </div>
</header>

<style>
  /* The sky darkens upward from a glowing horizon where the waves begin. */
  .hero {
    position: relative;
    height: 190px;
    overflow: hidden;
    background:
      radial-gradient(ellipse 70% 60% at 50% 100%, var(--sea-glow), transparent),
      linear-gradient(var(--sea-deep), var(--sea) 70%, var(--sea-glow));
  }

  /* Stars are tiny radial gradients over a repeating tile; only at night. */
  .stars {
    position: absolute;
    inset: 0 0 60px;
    display: none;
    background-repeat: repeat;
    animation: twinkle ease-in-out infinite alternate;
  }

  .near {
    background-image:
      radial-gradient(1.5px 1.5px at 12% 22%, var(--foam), transparent),
      radial-gradient(1.2px 1.2px at 38% 64%, var(--foam), transparent),
      radial-gradient(1.6px 1.6px at 61% 14%, var(--foam), transparent),
      radial-gradient(1.2px 1.2px at 84% 48%, var(--foam), transparent);
    background-size: 260px 130px;
    animation-duration: 4.2s;
  }

  .far {
    background-image:
      radial-gradient(1px 1px at 24% 42%, var(--foam-soft), transparent),
      radial-gradient(1px 1px at 52% 80%, var(--foam-soft), transparent),
      radial-gradient(1px 1px at 72% 30%, var(--foam-soft), transparent),
      radial-gradient(1px 1px at 6% 70%, var(--foam-soft), transparent);
    background-size: 170px 110px;
    opacity: 0.7;
    animation-duration: 6.4s;
    animation-delay: -2s;
  }

  .moon {
    position: absolute;
    top: 24px;
    right: 40px;
    width: 46px;
    height: 46px;
    border-radius: 50%;
    background: radial-gradient(
      circle at 38% 35%,
      color-mix(in srgb, var(--sky-light) 40%, white),
      var(--sky-light) 60%
    );
    box-shadow:
      0 0 32px color-mix(in srgb, var(--sky-light) 45%, transparent),
      0 0 90px color-mix(in srgb, var(--sky-light) 20%, transparent);
  }

  /* The ship's rail the parrot perches on: a plank over turned posts. */
  .rail {
    position: absolute;
    bottom: 0;
    left: 18px;
    width: 168px;
    height: 44px;
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
    bottom: 34px;
    left: 46px;
  }

  .titles {
    position: absolute;
    top: 50%;
    right: 24px;
    left: 200px;
    transform: translateY(-60%);
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

  @keyframes twinkle {
    to {
      opacity: 0.35;
    }
  }

  @media (prefers-color-scheme: dark) {
    .stars {
      display: block;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .stars {
      animation: none;
    }
  }
</style>
