# selah

A terminal-based Bible reader. Offline-first, keyboard-driven, fast.

## Features

- **4-panel reader** — Books, Chapters, Verses, and Scripture in a clean terminal layout
- **Keyboard + mouse navigation** — Vim-style (`h/j/k/l`) keys and mouse click/scroll
- **Full-text search** — FTS5-powered search with BM25 ranking via `/` key
- **Bookmarks** — Save and navigate to bookmarked verses (`b`/`B`)
- **5 themes** — Slate, Midnight, Parchment, Gospel, Terminal — cycle with `t`
- **Animated splash screen** — Ichthys ASCII art with fade-in animation
- **Random verse** — `r` in the TUI or `selah random` from the CLI
- **Session persistence** — Reading position, theme, and translation restored on relaunch
- **Fully offline** — KJV Bible data is embedded in the binary; no network requests at runtime
- **Cross-platform** — macOS (ARM) and Linux (ARM)

## Installation

### Homebrew (private tap)

Requires a GitHub PAT with read access to the tap repo. Set `HOMEBREW_GITHUB_API_TOKEN` in your shell, then:

```bash
brew tap jeremycastanza/selah-tap git@github-personal:jeremycastanza/homebrew-selah-tap.git
brew install selah
```

### Shell installer

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/jeremycastanza/selah/releases/download/v0.1.0/selah-installer.sh | sh
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
| `B` | Open bookmarks |
| `v` | Open translation picker |
| `t` | Cycle theme |
| `r` | Random verse |
| `?` | Replay splash screen |
| `q` (twice) | Quit |

## Platform Support

| Platform | Status |
|----------|--------|
| macOS (ARM) | Supported |
| Linux (ARM) | Supported |
| macOS (x86) | Not built in CI; compiles from source |
| Linux (x86) | Not built in CI; compiles from source |
| Windows | Not supported (WSL may work) |

## Tech Stack

| Layer | Technology |
|-------|------------|
| Language | Rust (edition 2024) |
| TUI framework | [Ratatui](https://github.com/ratatui/ratatui) 0.29 + crossterm |
| Data | SQLite via [rusqlite](https://github.com/rusqlite/rusqlite) (bundled) |
| Bible data | [scrollmapper/bible_databases](https://github.com/scrollmapper/bible_databases) (MIT) |
| CLI | [clap](https://github.com/clap-rs/clap) |
| Distribution | [cargo-dist](https://github.com/axodotdev/cargo-dist) + private Homebrew tap |

## Docs

- [`docs/architecture.md`](docs/architecture.md) — System design
- [`docs/decisions.md`](docs/decisions.md) — Architecture Decision Records
- [`docs/technical/deployment.md`](docs/technical/deployment.md) — Build and release process

## License

MIT
