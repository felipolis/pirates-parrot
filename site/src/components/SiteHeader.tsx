import Image from 'next/image';
import styles from './SiteHeader.module.css';

interface Props {
  repositoryUrl: string;
  /** The parrot used as the logo. */
  logoSrc: string;
}

export function SiteHeader({ repositoryUrl, logoSrc }: Props) {
  return (
    <header id="top" className={styles.header}>
      <a className={styles.brand} href="#top">
        <Image src={logoSrc} alt="" width={34} height={34} />
        Pirate’s Parrot
      </a>
      <nav className={styles.nav} aria-label="Main">
        <a href="#how-it-works">How it works</a>
        <a href="#crew">The crew</a>
        <a href={repositoryUrl}>GitHub</a>
      </nav>
    </header>
  );
}
