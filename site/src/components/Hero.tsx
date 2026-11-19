import type { Release } from '@/lib/release';
import { DictationDemo } from './DictationDemo';
import { DownloadButton } from './DownloadButton';
import styles from './Hero.module.css';

interface Props {
  release: Release | null;
  repositoryUrl: string;
  /** The parrot shown in the demo. */
  parrotSrc: string;
}

export function Hero({ release, repositoryUrl, parrotSrc }: Props) {
  return (
    <section className={styles.hero} aria-labelledby="hero-title">
      <div className={styles.copy}>
        <h1 id="hero-title" className={styles.title}>
          Talk to your terminal.
          <span className={styles.second}>A parrot writes it down.</span>
        </h1>
        <p className={styles.lead}>
          A small parrot sits in the corner of your screen. Press a shortcut and say what you need:
          your words appear while you speak, then land wherever you were typing. Claude Code, Codex,
          your editor or a chat box.
        </p>
        <DownloadButton release={release} repositoryUrl={repositoryUrl} />
      </div>
      <div className={styles.demo}>
        <DictationDemo parrotSrc={parrotSrc} />
      </div>
    </section>
  );
}
