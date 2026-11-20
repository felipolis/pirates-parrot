import { faGithub } from '@fortawesome/free-brands-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import Image from 'next/image';
import styles from './SiteHeader.module.css';

interface Props {
  repositoryUrl: string;
  /** The parrot used as the logo. */
  logoSrc: string;
}

/** Floats over the opening scene. */
export function SiteHeader({ repositoryUrl, logoSrc }: Props) {
  return (
    <header id="top" className={styles.header}>
      <div className={styles.bar}>
        <a className={styles.brand} href="#top">
          <Image src={logoSrc} alt="" width={36} height={36} />
          Pirate’s Parrot
        </a>
        <nav className={styles.nav} aria-label="Main">
          <a href="#how-it-works">How it works</a>
          <a href="#crew">The crew</a>
          <a className={styles.github} href={repositoryUrl}>
            <FontAwesomeIcon icon={faGithub} />
            GitHub
          </a>
        </nav>
      </div>
    </header>
  );
}
