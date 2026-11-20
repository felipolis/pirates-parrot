import Image from 'next/image';
import type { Release } from '@/lib/release';
import styles from './Closing.module.css';
import { DownloadButton } from './DownloadButton';
import { Waves } from './Waves';

interface Props {
  release: Release | null;
  repositoryUrl: string;
  /** The parrot seeing the reader off. */
  parrotSrc: string;
}

/** The last call to action, repeating the download for readers who scrolled. */
export function Closing({ release, repositoryUrl, parrotSrc }: Props) {
  return (
    <section className={styles.scene} aria-labelledby="closing-title">
      <div className={`${styles.closing} reveal`}>
        <Image className={styles.parrot} src={parrotSrc} alt="" width={150} height={150} />
        <h2 id="closing-title" className={styles.title}>
          Give your terminal a parrot.
        </h2>
        <DownloadButton release={release} repositoryUrl={repositoryUrl} align="center" />
        <p className={styles.note}>
          The installers aren’t code-signed yet, so your system may ask you to confirm the first
          launch. <a href={`${repositoryUrl}#download`}>Here’s how.</a>
        </p>
      </div>
      <Waves />
    </section>
  );
}
