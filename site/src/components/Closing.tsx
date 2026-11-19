import type { Release } from '@/lib/release';
import styles from './Closing.module.css';
import { DownloadButton } from './DownloadButton';

interface Props {
  release: Release | null;
  repositoryUrl: string;
}

/** The last call to action, repeating the download for readers who scrolled. */
export function Closing({ release, repositoryUrl }: Props) {
  return (
    <section className={styles.closing} aria-labelledby="closing-title">
      <h2 id="closing-title" className={styles.title}>
        Give your terminal a parrot.
      </h2>
      <DownloadButton release={release} repositoryUrl={repositoryUrl} />
      <p className={styles.note}>
        The installers aren’t code-signed yet, so your system may ask you to confirm the first
        launch. <a href={`${repositoryUrl}#download`}>Here’s how.</a>
      </p>
    </section>
  );
}
