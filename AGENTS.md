# Agent guide

Instructions for AI coding agents (and a useful checklist for humans) working on Pirate's
Parrot. Read [README.md](README.md) first for what the app does, the project layout and the
commands.

## The quality bar

This project values clean, readable, professional code over speed of delivery. These rules are
not optional:

- **Fix root causes, never symptoms.** No workarounds, hacks, magic delays (`sleep` to "wait for
  things to settle"), retries that hide a bug, or special cases for one reported input. If the
  right fix is larger than the task, say so instead of patching around it.
- **Never silence a tool.** Don't add `#[allow(...)]`, `eslint-disable`, `@ts-ignore`,
  `@ts-expect-error`, `as any`, `svelte-ignore` or `--no-verify` to make a check pass. Fix the
  code. If a suppression is truly justified, it needs a comment explaining why, and the change
  must call it out.
- **Never swallow errors.** No empty `catch`, `.catch(() => {})`, `let _ =` on a `Result`, or
  `unwrap_or_default()` that hides a failure. Handle the error, propagate it, or log it with
  context at the right level, and surface failures the user must know about in the UI.
- **No dead code or leftovers.** Delete unused functions, exports, CSS classes, theme tokens,
  dependencies and files, along with commented-out code. Don't leave `TODO`s behind; finish the
  work or say what is missing.
- **Keep the structure.** Put code where its responsibility lives (see below), one concern per
  module or component. Don't create catch-all `utils`/`helpers` files.
- **Write for the next reader.** Clear names over comments; comments explain _why_, not what.
  Every Rust module starts with a `//!` doc comment, and public items get `///` docs. Keep
  comments true when the code changes.
- **Stay in scope.** Change what the task needs. Don't reformat, rename or "improve" unrelated
  code in the same change.

## Architecture rules

- **`crates/parrot-core`** is the speech engine: audio capture, recognition, model download and
  installation. It must not depend on Tauri or know about windows or UI.
- **`src-tauri`** is the desktop shell: windows, tray and menus, global shortcut, clipboard and
  paste, settings, and the commands and events the frontend uses. Window code lives in
  `src-tauri/src/windows/`, one module per window.
- **`src/`** is the Svelte frontend. Screens live in `widget/` and `settings/`, reusable pieces
  in `lib/ui/`, the mascot in `lib/parrot/`, and the backend bridge in `lib/api/`.
- **`site/`** is the presentation website, a separate Next.js project in the same pnpm
  workspace. It never imports the app's code: the mascot reaches it only as SVG files exported
  by `scripts/export-parrots.ts`, so the drawing has a single source.
- **The backend owns all state.** The frontend renders it from `get_snapshot` plus events, and
  changes it only through commands. Never duplicate backend state or logic in the frontend.
- **The bridge is typed on both sides.** When a command, event or serialized type changes in
  Rust, update `src/lib/api/types.ts`, `commands.ts` and `events.ts` in the same change.
- **Privacy is a feature.** Audio and text never leave the machine. The only network access of
  the app is the model download in `parrot-core`, verified against a pinned SHA-256. Don't add
  telemetry, analytics, remote fonts or any other network call, to the app or the website.
- **No database.** Persistent state is small JSON files written atomically through
  `persistence::JsonFile`.
- **The widget never takes focus.** Clicking the parrot must leave keyboard focus in the app the
  user is dictating into.

## Stack conventions

Follow the official guidance of each tool; when in doubt, check its documentation instead of
guessing.

### Rust

- The edition and the minimum Rust version are set once, in `[workspace.package]` of
  [`Cargo.toml`](Cargo.toml); write code for that edition. Move to a new edition with
  `cargo fix --edition` across the whole workspace, never crate by crate.
- Must pass `cargo fmt` and `cargo clippy --workspace --all-targets -- -D warnings`. Workspace
  lints forbid `unsafe` code.
- Errors are typed enums built with `thiserror`, one per crate (`parrot_core::Error`,
  `AppError`). No `unwrap`/`expect` on paths that can fail at runtime; reserve them for real
  invariants and explain the invariant.
- Never block the async runtime. Long or blocking work (model loading, downloads, finishing a
  dictation) runs in `spawn_blocking` or on its own thread.
- Unit tests live next to the code in a `#[cfg(test)] mod tests`. Add tests for logic that can
  be tested without hardware (layout math, parsing, state transitions).
- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) for naming and
  public APIs.

### Tauri 2

- Commands that create or show windows must be `async`; synchronous commands that do so deadlock
  on Windows.
- Grant the minimum permissions in `src-tauri/capabilities/` and keep the CSP in
  `tauri.conf.json` strict. Window labels must match the capability file.
- Prefer official Tauri plugins over hand-written platform code.
- The app icon comes from `src-tauri/icons/icon.svg`; regenerate the platform icons as its
  header comment describes instead of editing them by hand.

### Svelte 5 and TypeScript

- Use runes only: `$state`, `$derived`, `$effect`, `$props` and snippets. No legacy syntax
  (`export let`, `$:`, `<slot>`, `createEventDispatcher`) and no Svelte stores for app state.
- Prefer `$derived` over `$effect`. Every `$effect` that starts a timer, listener or
  subscription returns its cleanup.
- TypeScript is strict; type everything that crosses the bridge.
- Styles are scoped to their component and use the theme tokens in `src/styles/base.css`
  (colors, fonts, radii) instead of new hard-coded values; add a token when a new color is
  needed. Only illustration details, such as the mascot's eyes, keep fixed colors. Keep both
  light and dark themes working.
- Keep the UI accessible: real `<button>`s for actions, labels for every control, and respect
  `prefers-reduced-motion` for animations.
- UI text is in English and follows the pirate theme without hurting clarity.

### Next.js (the website)

- Use the App Router with Server Components by default; add `'use client'` only to components
  that need the browser, and keep them small.
- The site is a static export (`output: 'export'`): no server-only features such as Server
  Actions, rewrites, cookies or on-request image optimization. Data is read at build time.
- Read browser APIs (`navigator`, `matchMedia`) through `useSyncExternalStore`, not by setting
  state in an effect, so the static HTML and the first render agree.
- Style with CSS Modules and the tokens in `site/src/app/globals.css`, which mirror the app's
  theme. Load fonts with `next/font` and images with `next/image`.
- Lint with the ESLint CLI and `eslint-config-next` (`next lint` no longer exists).
- Copy is in English, plain and specific. No filler, buzzwords or claims the app can't back up.

### Dependencies

- Add a dependency only when it clearly beats writing the code, is well maintained and has a
  compatible license. Explain the choice in the change.
- Commit `Cargo.lock` and `pnpm-lock.yaml` with dependency changes. Use `pnpm`, never `npm` or
  `yarn`.

## Before you finish

Run every check and make sure it passes with zero warnings:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
pnpm format:check
pnpm lint
pnpm check
pnpm --filter pirates-parrot-site lint
pnpm --filter pirates-parrot-site typecheck
pnpm --filter pirates-parrot-site build
```

Then:

- For anything visible, run the app with `pnpm tauri dev` and confirm the change works,
  including error states, in both the widget and the settings window when relevant. For the
  website, check it with `pnpm --filter pirates-parrot-site dev` at desktop and phone widths.
- Update [README.md](README.md) when behavior, setup or commands change.
- Report honestly what was verified and what was not (for example macOS and Linux, which may
  not be testable locally).
