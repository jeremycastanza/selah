# selah

A terminal-based Bible reader. Offline-first, keyboard-driven, fast.

## Features

- **Multiple translations** — 140+ translations via bundled SQLite database
- **Full-text search** — FTS5-powered search with BM25 ranking, no network required
- **Fully offline** — all data is embedded in the binary; no API calls at runtime
- **Cross-platform** — macOS and Linux

## Installation

> Distribution is via a private Homebrew tap. Requires [Homebrew](https://brew.sh) and the [GitHub CLI](https://cli.github.com) authenticated.

```bash
brew install <tap>/selah
```

## Usage

> Coming soon.

## Building from Source

Requires the [Rust toolchain](https://rustup.rs).

```bash
cargo build --release
```

The binary will be at `target/release/selah`.

## Platform Support

| Platform | Status                       |
| -------- | ---------------------------- |
| macOS    | Supported                    |
| Linux    | Supported                    |
| Windows  | Not supported (WSL may work) |

## Tech Stack

| Layer         | Technology                                                                            |
| ------------- | ------------------------------------------------------------------------------------- |
| Language      | Rust                                                                                  |
| TUI framework | [Ratatui](https://github.com/ratatui/ratatui) + crossterm                             |
| Data          | SQLite via [rusqlite](https://github.com/rusqlite/rusqlite) (bundled)                 |
| Bible data    | [scrollmapper/bible_databases](https://github.com/scrollmapper/bible_databases) (MIT) |

## Docs

- [`docs/architecture.md`](docs/architecture.md) — System design
- [`docs/decisions.md`](docs/decisions.md) — Architecture Decision Records
- [`docs/technical/deployment.md`](docs/technical/deployment.md) — Build and release process

## License

TBD
