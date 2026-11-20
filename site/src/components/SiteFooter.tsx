import { faGithub } from '@fortawesome/free-brands-svg-icons';
import {
  faBug,
  faScaleBalanced,
  faTag,
  type IconDefinition,
} from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import styles from './SiteFooter.module.css';

interface Props {
  repositoryUrl: string;
  releasesUrl: string;
}

export function SiteFooter({ repositoryUrl, releasesUrl }: Props) {
  const links: { label: string; href: string; icon: IconDefinition }[] = [
    { label: 'Source code', href: repositoryUrl, icon: faGithub },
    { label: 'Releases', href: releasesUrl, icon: faTag },
    { label: 'Report a problem', href: `${repositoryUrl}/issues`, icon: faBug },
    { label: 'MIT license', href: `${repositoryUrl}/blob/main/LICENSE`, icon: faScaleBalanced },
  ];

  return (
    <footer className={styles.footer}>
      <nav className={styles.links} aria-label="Project">
        {links.map(({ label, href, icon }) => (
          <a key={label} href={href}>
            <FontAwesomeIcon icon={icon} className={styles.icon} />
            {label}
          </a>
        ))}
      </nav>
      <p>
        Speech recognition by NVIDIA Nemotron 3.5 ASR Streaming, run on your computer with
        sherpa-onnx. Built with Tauri and Svelte. Icons by Font Awesome (CC BY 4.0).
      </p>
    </footer>
  );
}
