import { resolve } from 'node:path';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vite';

// Set by `tauri dev` when developing against a device on the network.
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [svelte()],
  // Keep Rust compiler errors visible in the terminal.
  clearScreen: false,
  server: {
    // Tauri expects a fixed port and fails fast if it is taken.
    port: 1420,
    strictPort: true,
    host: host ?? '127.0.0.1',
    hmr: host ? { protocol: 'ws', host, port: 1421 } : undefined,
    watch: { ignored: ['**/src-tauri/**', '**/crates/**', '**/target/**', '**/site/**'] },
  },
  build: {
    target: 'es2022',
    rollupOptions: {
      // One page per window.
      input: {
        widget: resolve(import.meta.dirname, 'widget.html'),
        settings: resolve(import.meta.dirname, 'settings.html'),
      },
    },
  },
});
