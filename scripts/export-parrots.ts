// Renders every parrot skin of the desktop app to a standalone, animated SVG
// file, plus a manifest describing them. The website shows the real mascot
// this way, without keeping a second copy of the drawing.
//
// Usage: pnpm run export-parrots <output-dir>

import { mkdir, rm, writeFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import type { Component } from 'svelte';
import type * as SvelteServer from 'svelte/server';
import { createServer } from 'vite';
import type { Mood } from '../src/lib/parrot/mood';
import type * as Skins from '../src/lib/parrot/skins';
import type { ParrotSkin } from '../src/lib/parrot/skins';

/** One entry of `index.json`, read by the website. */
interface ExportedParrot {
  id: string;
  name: string;
  tagline: string;
  file: string;
}

interface ParrotProps {
  skin: ParrotSkin;
  mood?: Mood;
  size?: number;
}

const outputArgument = process.argv[2];
if (!outputArgument) {
  console.error('Usage: pnpm run export-parrots <output-dir>');
  process.exit(1);
}
const outputDir = resolve(outputArgument);

// Compile the components with their CSS injected, so it can be embedded in
// each file and the parrots keep their animations outside the app.
const vite = await createServer({
  configFile: false,
  root: resolve(import.meta.dirname, '..'),
  plugins: [svelte({ compilerOptions: { css: 'injected' } })],
  server: { middlewareMode: true },
  appType: 'custom',
  logLevel: 'warn',
});

try {
  const { render } = (await vite.ssrLoadModule('svelte/server')) as typeof SvelteServer;
  const { SKINS } = (await vite.ssrLoadModule('/src/lib/parrot/skins.ts')) as typeof Skins;
  const Parrot = (await vite.ssrLoadModule('/src/lib/parrot/Parrot.svelte'))
    .default as Component<ParrotProps>;

  await rm(outputDir, { recursive: true, force: true });
  await mkdir(outputDir, { recursive: true });

  const manifest: ExportedParrot[] = [];
  for (const skin of SKINS) {
    const { head, body } = render(Parrot, { props: { skin, mood: 'idle', size: 100 } });
    const file = `${skin.id}.svg`;
    await writeFile(join(outputDir, file), standaloneSvg(head, body));
    manifest.push({ id: skin.id, name: skin.name, tagline: skin.tagline, file });
  }
  await writeFile(join(outputDir, 'index.json'), `${JSON.stringify(manifest, null, 2)}\n`);
  console.log(`Exported ${manifest.length} parrots to ${outputDir}`);
} finally {
  await vite.close();
}

/** Moves the component's styles inside its `<svg>`, which becomes the whole file. */
function standaloneSvg(head: string, body: string): string {
  const css = [...head.matchAll(/<style[^>]*>([\s\S]*?)<\/style>/g)]
    .map((match) => match[1])
    .join('\n');
  const svg = /<svg\b[\s\S]*<\/svg>/.exec(body)?.[0];
  if (!svg) throw new Error('The parrot component did not render an <svg> element.');
  return `${svg.replace(/^<svg\b[^>]*>/, (openingTag) => `${openingTag}<style>${css}</style>`)}\n`;
}
