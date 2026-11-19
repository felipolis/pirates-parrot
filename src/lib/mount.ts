import '@fontsource-variable/nunito/wght.css';
import '@fontsource/im-fell-english/latin-400.css';
import '@fontsource/im-fell-english/latin-400-italic.css';
import '@fontsource/im-fell-english-sc/latin-400.css';
import '@fontsource/pirata-one/latin-400.css';
import '../styles/base.css';
import { mount, type Component } from 'svelte';

/** Mounts a window's root component into the page's `#app` element. */
export function mountWindow(root: Component): void {
  const target = document.getElementById('app');
  if (!target) throw new Error('The page is missing its #app element.');

  // The webview's default context menu (reload, inspect…) has no place in a
  // desktop app.
  document.addEventListener('contextmenu', (event) => event.preventDefault());

  mount(root, { target });
}
