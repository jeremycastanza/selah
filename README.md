# selah

[![CI](https://github.com/jeremycastanza/selah/actions/workflows/release.yml/badge.svg)](https://github.com/jeremycastanza/selah/actions/workflows/release.yml)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue.svg)](LICENSE)

A terminal-based Bible reader. Offline-first, keyboard-driven, fast.

## Features

- **4-panel reader** — Books, Chapters, Verses, and Scripture in a clean terminal layout
- **Keyboard + mouse navigation** — Vim-style (`h/j/k/l`) keys and mouse click/scroll
- **Full-text search** — FTS5-powered search with BM25 ranking via `/` key
- **Bookmarks** — Save and navigate to bookmarked verses (`b`/`B`)
- **Highlights** — Color-highlight verses with toggle visibility (`H`/`g`/`G`)
- **Notes** — Attach notes to any verse with a multi-line editor (`n`/`N`)
- **5 themes** — Slate, Midnight, Parchment, Gospel, Terminal — cycle with `t`
- **Help menu** — Tabbed overlay with full keybinding reference (`?`)
- **Multiple translations** — KJV bundled offline; additional translations via YouVersion API
- **Animated splash screen** — Ichthys ASCII art with fade-in animation
- **Random verse** — `r` in the TUI or `selah random` from the CLI
- **Session persistence** — Reading position, theme, and translation restored on relaunch
- **Fully offline** — KJV Bible data is embedded in the binary; no network requests at runtime
- **Cross-platform** — macOS (ARM) and Linux (ARM)

## Installation

### Homebrew

```bash
brew tap jeremycastanza/selah
brew install selah
```

### Shell installer

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/jeremycastanza/selah/releases/latest/download/selah-installer.sh | sh
```

### Build from source

Requires the [Rust toolchain](https://rustup.rs) (edition 2024).

```bash
cargo build --release
# Binary: target/release/selah
```

## Usage

```bash
selah              # launch the TUI
selah random       # print a random verse to stdout
selah --no-banner  # skip the splash screen
```

### Keybindings

| Key | Action |
|-----|--------|
| `h/j/k/l` or arrows | Navigate panels and lists |
| `Enter` | Select / confirm |
| `/` | Open search |
| `b` | Bookmark current verse |
| `B` | Open bookmarks list |
| `H` | Highlight current verse |
| `g` | Toggle highlight visibility |
| `G` | Open highlights list |
| `n` | Add/edit note on current verse |
| `N` | Open notes list |
| `v` | Open translation picker |
| `t` | Cycle theme |
| `r` | Random verse |
| `S` | Open settings |
| `?` | Open help menu |
| `q` | Quit (with confirmation) |
| `Esc` | Close overlay |

## Platform Support

| Platform | Status |
|----------|--------|
| macOS (ARM) | Supported |
| Linux (ARM) | Supported |
| macOS (x86) | Not built in CI; compiles from source |
| Linux (x86) | Not built in CI; compiles from source |
| Windows | Not supported (WSL may work) |

## Additional Translations

Selah bundles the KJV for fully offline use. To access additional translations (ESV, NIV, NLT, etc.), you need a [YouVersion Platform](https://developers.youversion.com) API key.

### Getting a YouVersion Platform App Key

1. Go to [developers.youversion.com](https://developers.youversion.com)
2. Sign in or create an account
3. Create a new application
4. Copy your **App Key** from the application dashboard

### Adding Your Key to Selah

**Option A — In the app:**

1. Press `S` to open Settings
2. Press `K` to edit the API key
3. Paste your key and press `Enter`

**Option B — Environment variable:**

```bash
export SELAH_YVP_APP_KEY="your-app-key-here"
```

Once configured, press `v` to open the translation picker and select from available translations.

## Tech Stack

| Layer | Technology |
|-------|------------|
| Language | Rust (edition 2024) |
| TUI framework | [Ratatui](https://github.com/ratatui/ratatui) 0.29 + crossterm |
| Data | SQLite via [rusqlite](https://github.com/rusqlite/rusqlite) (bundled) |
| Bible data | [scrollmapper/bible_databases](https://github.com/scrollmapper/bible_databases) (MIT) |
| CLI | [clap](https://github.com/clap-rs/clap) |
| Distribution | [cargo-dist](https://github.com/axodotdev/cargo-dist) + Homebrew tap |

## Docs

- [`docs/architecture.md`](docs/architecture.md) — System design
- [`docs/decisions.md`](docs/decisions.md) — Architecture Decision Records
- [`docs/technical/deployment.md`](docs/technical/deployment.md) — Build and release process

## License

GPL-3.0
