'use client';

// The main call to action: the installer for the visitor's system, picked in
// the browser. The static HTML links to the release page until then.

import type { Download, Platform, Release } from '@/lib/release';
import { type OperatingSystem, useVisitorOs } from '@/lib/visitor-os';
import styles from './DownloadButton.module.css';

interface Offer {
  system: string;
  /** Offered on the button. */
  main: Platform;
  /** Offered as links below it, for the same system. */
  others: Platform[];
}

const OFFERS: Record<OperatingSystem, Offer> = {
  windows: { system: 'Windows', main: 'windows', others: [] },
  // Browsers do not reveal the processor, and new Macs use Apple silicon.
  macos: { system: 'macOS', main: 'macos-apple-silicon', others: ['macos-intel'] },
  linux: { system: 'Linux', main: 'linux-appimage', others: ['linux-deb', 'linux-rpm'] },
};

interface Props {
  release: Release | null;
  repositoryUrl: string;
}

export function DownloadButton({ release, repositoryUrl }: Props) {
  const os = useVisitorOs();

  if (!release) {
    return (
      <div className={styles.download}>
        <a className={styles.button} href={repositoryUrl}>
          Follow it on GitHub
        </a>
        <p className={styles.meta}>The first release is on its way.</p>
      </div>
    );
  }

  const offer = os ? OFFERS[os] : null;
  const main = offer && findDownload(release.downloads, offer.main);
  const others = (offer?.others ?? []).flatMap((platform) => {
    const download = findDownload(release.downloads, platform);
    return download ? [download] : [];
  });

  return (
    <div className={styles.download}>
      <a className={styles.button} href={main?.url ?? release.url}>
        {main && offer ? `Download for ${offer.system}` : 'Download'}
      </a>
      <p className={styles.meta}>
        Version {release.version}
        {main ? ` · ${formatMegabytes(main.sizeBytes)}` : null}
        {others.map((download) => (
          <span key={download.platform}>
            {' · '}
            <a href={download.url}>{download.label}</a>
          </span>
        ))}
        {' · '}
        <a href={release.url}>All platforms</a>
      </p>
    </div>
  );
}

function findDownload(downloads: Download[], platform: Platform): Download | undefined {
  return downloads.find((download) => download.platform === platform);
}

function formatMegabytes(bytes: number): string {
  return `${Math.round(bytes / 1_000_000)} MB`;
}
