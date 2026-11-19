// The latest published release of the desktop app, read from GitHub when the
// site is built. The release workflow rebuilds the site whenever a new version
// is published, so the download links are always current.

import { REPOSITORY } from './repository';

export type Platform =
  'windows' | 'macos-apple-silicon' | 'macos-intel' | 'linux-appimage' | 'linux-deb' | 'linux-rpm';

export interface Download {
  platform: Platform;
  /** What the visitor is offered, e.g. "macOS (Apple silicon)". */
  label: string;
  url: string;
  sizeBytes: number;
}

export interface Release {
  version: string;
  url: string;
  downloads: Download[];
}

/**
 * How each platform's installer is recognized among the release assets, by the
 * file names Tauri gives them. Architectures are explicit because a release may
 * also carry ARM builds.
 */
const INSTALLERS: { platform: Platform; label: string; matches: (name: string) => boolean }[] = [
  { platform: 'windows', label: 'Windows', matches: (name) => name.endsWith('_x64-setup.exe') },
  {
    platform: 'macos-apple-silicon',
    label: 'macOS (Apple silicon)',
    matches: (name) => name.endsWith('_aarch64.dmg'),
  },
  { platform: 'macos-intel', label: 'macOS (Intel)', matches: (name) => name.endsWith('_x64.dmg') },
  {
    platform: 'linux-appimage',
    label: 'Linux (AppImage)',
    matches: (name) => name.endsWith('_amd64.AppImage'),
  },
  {
    platform: 'linux-deb',
    label: 'Linux (Debian, Ubuntu)',
    matches: (name) => name.endsWith('_amd64.deb'),
  },
  {
    platform: 'linux-rpm',
    label: 'Linux (Fedora)',
    matches: (name) => name.endsWith('.x86_64.rpm'),
  },
];

/** The parts of GitHub's release payload the site uses. */
interface GitHubRelease {
  tag_name: string;
  html_url: string;
  assets: { name: string; browser_download_url: string; size: number }[];
}

/**
 * The latest release, or `null` while none has been published. Any other
 * failure stops the build, so a broken page is never deployed.
 */
export async function getLatestRelease(): Promise<Release | null> {
  const token = process.env.GITHUB_TOKEN;
  const response = await fetch(`https://api.github.com/repos/${REPOSITORY}/releases/latest`, {
    headers: {
      Accept: 'application/vnd.github+json',
      'X-GitHub-Api-Version': '2022-11-28',
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
  });
  if (response.status === 404) return null;
  if (!response.ok) {
    throw new Error(`GitHub answered ${response.status} when asked for the latest release.`);
  }

  const release = (await response.json()) as GitHubRelease;
  return {
    version: release.tag_name.replace(/^v/, ''),
    url: release.html_url,
    downloads: INSTALLERS.flatMap(({ platform, label, matches }) => {
      const asset = release.assets.find((candidate) => matches(candidate.name));
      return asset
        ? [{ platform, label, url: asset.browser_download_url, sizeBytes: asset.size }]
        : [];
    }),
  };
}
