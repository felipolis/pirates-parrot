// The parrots of the desktop app, exported from its own Svelte component by
// `pnpm run parrots` into `public/parrots/`.

import { readFile } from 'node:fs/promises';
import { join } from 'node:path';

export interface Parrot {
  id: string;
  name: string;
  tagline: string;
  /** Public path of the animated SVG. */
  src: string;
}

interface ManifestEntry {
  id: string;
  name: string;
  tagline: string;
  file: string;
}

const PARROTS_DIR = join(process.cwd(), 'public', 'parrots');

/** Every parrot, in the order the app lists them. Read at build time. */
export async function getCrew(): Promise<Parrot[]> {
  const manifest = JSON.parse(
    await readFile(join(PARROTS_DIR, 'index.json'), 'utf8'),
  ) as ManifestEntry[];
  return manifest.map(({ id, name, tagline, file }) => ({
    id,
    name,
    tagline,
    src: `/parrots/${file}`,
  }));
}
