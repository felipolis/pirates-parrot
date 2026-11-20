import { faGithub } from '@fortawesome/free-brands-svg-icons';
import { faAnchor } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import type { Release } from '@/lib/release';
import { DictationDemo } from './DictationDemo';
import { DownloadButton } from './DownloadButton';
import styles from './Hero.module.css';
import { NightSky } from './NightSky';
import { Waves } from './Waves';

interface Props {
  release: Release | null;
  repositoryUrl: string;
  /** The parrot shown in the demo. */
  parrotSrc: string;
}

/** The opening scene: a night at sea, with the parrot at work in a terminal. */
export function Hero({ release, repositoryUrl, parrotSrc }: Props) {
  return (
    <section className={styles.scene} aria-labelledby="hero-title">
      <NightSky />
      <div className={styles.hero}>
        <div className={styles.copy}>
          <p className={styles.motto}>
            <FontAwesomeIcon icon={faAnchor} />
            Free, open source and offline
          </p>
          <h1 id="hero-title" className={styles.title}>
            Talk to your terminal.
            <span className={styles.second}>A parrot writes it down.</span>
          </h1>
          <p className={styles.lead}>
            A small parrot sits in the corner of your screen. Press a shortcut and say what you
            need: your words appear while you speak, then land wherever you were typing.
          </p>
          <DownloadButton
            release={release}
            repositoryUrl={repositoryUrl}
            secondary={
              <a className={styles.source} href={repositoryUrl}>
                <FontAwesomeIcon icon={faGithub} />
                View the code
              </a>
            }
          />
        </div>
        <div className={styles.demo}>
          <DictationDemo parrotSrc={parrotSrc} />
        </div>
      </div>
      <Waves />
    </section>
  );
}
