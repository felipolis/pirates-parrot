<p align="center">
  <img src="src-tauri/icons/128x128@2x.png" width="128" alt="Captain Scarlet, the Pirate's Parrot mascot" />
</p>

<h1 align="center">Pirate's Parrot</h1>

<p align="center">
  Talk to your terminal. A parrot writes it down.
</p>

Pirate's Parrot is a small desktop widget that turns your voice into text **in real time,
entirely on your computer**. Press a shortcut (or click the parrot), say what you want, and the
words appear as you speak. When you stop, the text is pasted into whatever you were typing in:
a terminal running Claude Code or Codex, a chat box, an editor.

- **Live transcription.** Words show up while you talk, not after.
- **Private by design.** Audio never leaves your machine. No account, no cloud, no database.
- **Stays out of the way.** A parrot in the corner of your screen that never steals focus.
- **Multilingual.** English, Portuguese, Spanish, French, German, Japanese and more, with
  automatic language detection, punctuation and capitalization.
- **Pick your crewmate.** Captain Scarlet, Blue Jack, Cabin Boy, Old Salt and Phantom Polly.

## Download

Grab the installer for your system from the [latest release](../../releases/latest):

| System                | File                                                    |
| --------------------- | ------------------------------------------------------- |
| Windows               | `…_x64-setup.exe` (or the `.msi`)                       |
| macOS (Apple silicon) | `…_aarch64.dmg`                                         |
| macOS (Intel)         | `…_x64.dmg`                                             |
| Linux                 | `.deb` (Debian, Ubuntu), `.rpm` (Fedora) or `.AppImage` |

Pirate's Parrot is not code-signed yet, so your system will warn you the first time:

- **Windows:** on "Windows protected your PC", click **More info**, then **Run anyway**.
- **macOS:** after the first attempt to open it, go to System Settings → Privacy & Security and
  click **Open Anyway**. If macOS says the app is damaged, run
  `xattr -dr com.apple.quarantine "/Applications/Pirate’s Parrot.app"` in the Terminal.
- **Linux:** make the AppImage executable with `chmod +x`, or install the package with your
  package manager (for example `sudo apt install ./<downloaded-file>.deb`).

On first launch the settings window opens so you can download the speech model (about 475 MB).
After that the app works offline.

### Uninstalling

Uninstall the app as usual for your system. The speech model, your settings and the logs stay
behind in case you reinstall; delete these folders to remove them too:

| System  | Settings                                                | Speech model and logs                                                       |
| ------- | ------------------------------------------------------- | --------------------------------------------------------------------------- |
| Windows | `%APPDATA%\io.github.piratesparrot`                     | `%LOCALAPPDATA%\io.github.piratesparrot`                                    |
| macOS   | `~/Library/Application Support/io.github.piratesparrot` | The same folder, plus `~/Library/Logs/io.github.piratesparrot` for the logs |
| Linux   | `~/.config/io.github.piratesparrot`                     | `~/.local/share/io.github.piratesparrot`                                    |

## How it works

Speech recognition runs on
[NVIDIA Nemotron 3.5 ASR Streaming](https://huggingface.co/nvidia/nemotron-3.5-asr-streaming-0.6b),
a 600M-parameter cache-aware streaming model, executed on the CPU by
[sherpa-onnx](https://github.com/k2-fsa/sherpa-onnx). It processes each new slice of audio once,
which is what makes real-time transcription possible on an ordinary laptop: about ten times
faster than real time on a modern CPU, no GPU required.

The model (about 475 MB) is downloaded once, from the settings window, from the official
sherpa-onnx release and verified against a pinned SHA-256 checksum. Three variants trade latency for
accuracy:

| Variant  | Chunk  | Best for                                 |
| -------- | ------ | ---------------------------------------- |
| Fast     | 160 ms | Seeing words the instant you say them    |
| Balanced | 560 ms | Everyday use (default)                   |
| Accurate | 1.12 s | Long dictations where every word matters |

## Using it

1. Launch Pirate's Parrot. On first run the settings window opens: click **Download model**.
2. Press <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>Space</kbd> (<kbd>⌘</kbd>+<kbd>⇧</kbd>+<kbd>Space</kbd>
   on macOS) or click the parrot, and start talking.
3. Press the shortcut again or click the parrot to finish. The text is pasted where your cursor
   is and also copied to the clipboard.

Drag the parrot anywhere; it remembers the spot. Right-click it, or use the tray icon, to hide it
or open the settings, where you can change the shortcut (toggle or hold-to-talk), language,
microphone, model and parrot. A hidden parrot still pops up while it listens to you, then goes
away again.

> **Tip:** automatic language detection produces the best punctuation. Pick a fixed language only
> if detection guesses wrong for you.

## Development

Contributions are welcome. Read [AGENTS.md](AGENTS.md) first: it holds the project's quality bar
and conventions, for people and AI coding agents alike.

### Prerequisites

- [Rust](https://rustup.rs) (stable, 1.90 or newer)
- [Node.js](https://nodejs.org) 24 and [pnpm](https://pnpm.io)
- The [Tauri prerequisites](https://tauri.app/start/prerequisites/) for your platform. On Linux
  also `libasound2-dev` (microphone) and `libxdo-dev` (pasting).

The sherpa-onnx native libraries are downloaded automatically by its build script on the first
build.

### Commands

```sh
pnpm install          # install frontend dependencies
pnpm tauri dev        # run the app in development mode
pnpm tauri build      # build optimized installers into target/release/bundle

cargo test --workspace                                   # Rust tests
cargo clippy --workspace --all-targets -- -D warnings    # Rust lints
pnpm check && pnpm lint && pnpm format:check             # frontend checks
```

You don't need to build an installer to try your changes. Keep `pnpm tauri dev` running while you
work: changes to the frontend (`src/`) show up instantly, and changes to the Rust code rebuild
and restart the app on their own. The first run takes a few minutes to compile everything; later
rebuilds take seconds. Use `pnpm tauri build` only to produce the installers.

To try the speech engine without the app, transcribe a WAV file with an installed model:

```sh
cargo run --release -p parrot-core --example transcribe_wav -- <model-dir> <audio.wav> [pt-BR]
```

### Project layout

```
crates/parrot-core/   Speech engine: microphone capture, streaming recognition,
                      model download and installation. No UI code.
src-tauri/            Desktop shell: windows, tray, global shortcut, clipboard and
                      the commands/events the frontend uses.
src/                  Svelte frontend
  lib/api/            Typed bindings to the backend commands and events
  lib/parrot/         The animated SVG mascot and its skins
  widget/             The floating parrot and its speech bubble
  settings/           The settings window
site/                 The presentation website (Next.js), deployed to Cloudflare
scripts/              Repository tools, such as exporting the parrots for the website
```

The backend owns all state. The frontend renders it from a snapshot plus events
(`engine-status`, `dictation-phase`, `transcript`, `level`, `dictation-result`,
`settings-changed`, `shortcut-problem`), and changes it only through commands. Settings and
the widget's position live in small JSON files in the OS config folder; models in the OS local
data folder.

### Adding a parrot

Skins are plain data in [`src/lib/parrot/skins.ts`](src/lib/parrot/skins.ts): a palette plus a
list of accessories (`tricorne`, `bandana`, `eyepatch`, `earring`). Add an entry and it shows up
in the settings, and on the website the next time it is built.

### The website

The presentation website in [`site/`](site) is a Next.js app exported as static files and served
by Cloudflare Workers. It shows the app's real parrots: `pnpm run parrots` renders them from the
Svelte component into `site/public/parrots/`, which happens automatically before `dev` and
`build`.

```sh
cp site/.env.example site/.env.local     # set GITHUB_REPOSITORY to owner/name
pnpm --filter pirates-parrot-site dev    # run it at http://localhost:3000
pnpm --filter pirates-parrot-site build  # build it into site/out
pnpm --filter pirates-parrot-site lint && pnpm --filter pirates-parrot-site typecheck
```

The download buttons read the latest GitHub release while the site builds.
[`site.yml`](.github/workflows/site.yml) checks the site on every pull request that touches
it, and deploys it when `main` changes and whenever a release is published. Deploying needs
two repository secrets: `CLOUDFLARE_API_TOKEN` (a token with the _Edit Cloudflare Workers_
permission) and `CLOUDFLARE_ACCOUNT_ID`.

### Commits and releases

Commit messages follow [Conventional Commits](https://www.conventionalcommits.org), because
releases are computed from them:

| Commit                          | Effect on the next version                       |
| ------------------------------- | ------------------------------------------------ |
| `fix: …`                        | Patch release (0.1.0 → 0.1.1)                    |
| `feat: …`                       | Minor release (0.1.0 → 0.2.0)                    |
| `feat!: …` or `BREAKING CHANGE` | Minor before 1.0 (0.1.0 → 0.2.0), major after it |
| `docs:`, `ci:`, `refactor:`, …  | No release on its own                            |

Releases are automated by [release-please](https://github.com/googleapis/release-please) in
[`release.yml`](.github/workflows/release.yml):

1. After every push to `main`, release-please opens or updates a release pull request. It bumps
   the version in `package.json`, `Cargo.toml` and `Cargo.lock` and adds the changes to
   [`CHANGELOG.md`](CHANGELOG.md).
2. Merge it when you want to ship. The workflow then builds the installers for Windows, macOS
   (Apple silicon and Intel) and Linux, publishes the release with them attached, and
   redeploys the website so its download buttons offer the new version.

Never edit the version numbers or create release tags by hand. The repository needs _Settings →
Actions → General → Allow GitHub Actions to create and approve pull requests_ turned on for
release-please to open its pull request.

## Troubleshooting

- **Windows blocks the build or the app.** Smart App Control refuses unsigned executables,
  including Rust build scripts. Development requires turning it off; release builds need code
  signing to run on machines that have it on.
- **macOS: text is copied but not pasted.** Grant Pirate's Parrot the Accessibility permission in
  System Settings → Privacy & Security, which macOS requires to send the paste keystroke.
- **Linux on Wayland:** global shortcuts and simulated paste depend on the compositor. Copy mode
  works everywhere.

## Credits and licenses

Pirate's Parrot is released under the [MIT license](LICENSE).

- Speech model: NVIDIA Nemotron 3.5 ASR Streaming 0.6B, under the OpenMDW-1.1 license, which
  permits commercial use. The model is downloaded at runtime and is not redistributed here.
- Inference: [sherpa-onnx](https://github.com/k2-fsa/sherpa-onnx) (Apache-2.0) and ONNX Runtime.
- App shell: [Tauri](https://tauri.app) and [Svelte](https://svelte.dev).
