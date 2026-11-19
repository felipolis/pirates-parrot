import styles from './SiteFooter.module.css';

interface Props {
  repositoryUrl: string;
  releasesUrl: string;
}

export function SiteFooter({ repositoryUrl, releasesUrl }: Props) {
  return (
    <footer className={styles.footer}>
      <nav className={styles.links} aria-label="Project">
        <a href={repositoryUrl}>Source code</a>
        <a href={releasesUrl}>Releases</a>
        <a href={`${repositoryUrl}/issues`}>Report a problem</a>
        <a href={`${repositoryUrl}/blob/main/LICENSE`}>MIT license</a>
      </nav>
      <p>
        Speech recognition by NVIDIA Nemotron 3.5 ASR Streaming, run on your computer with
        sherpa-onnx. Built with Tauri and Svelte.
      </p>
    </footer>
  );
}
