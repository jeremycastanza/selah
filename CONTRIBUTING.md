# Contributing to Selah

Thank you for your interest in contributing to Selah, a fully offline terminal Bible reader built in Rust.

## Prerequisites

- **Rust toolchain** — Install via [rustup](https://rustup.rs/). The project uses Rust edition 2024.
- **macOS or Linux** — Primary development platforms.
- No other runtime dependencies. `rusqlite` bundles SQLite from source.

## Getting Started

```bash
git clone <repo-url>
cd selah
cargo build
cargo run
```

## Development Commands

| Command | Purpose |
|---------|---------|
| `cargo build` | Debug build |
| `cargo run` | Run the TUI |
| `cargo run -- --no-banner` | Skip splash screen |
| `cargo run -- random` | Print a random verse to stdout |
| `cargo test` | Run all unit tests |
| `cargo clippy -- -D warnings` | Lint (must be clean) |
| `cargo fmt --check` | Check formatting |
| `cargo fmt` | Auto-format |
| `cargo build --release` | Optimized binary → `target/release/selah` |

## Project Structure

```
src/
├── app.rs          # Application state, event loop, key dispatch
├── main.rs         # CLI entry point (clap)
├── bible/          # Bible data: books, DB queries, types, translations
├── config/         # User data persistence: bookmarks, highlights, notes, session
└── ui/             # All rendering: browser panels, overlays, themes
data/               # Bundled KJV SQLite database
docs/               # Documentation (see below)
```

See `docs/workflow/context.md` for the full architecture overview.

## Making Changes

1. **Branch from `main`** — Never commit directly to `main`.
2. **Use conventional commits** — `feat(scope): description`, `fix(scope): description`, etc.
3. **Open a PR** with a clear title and description. Reference the issue number.
4. **Ensure all checks pass** before requesting review:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

## Code Standards

- **No `unsafe` Rust** anywhere in the codebase.
- **No runtime network requests** — All Bible data is bundled in the binary via `include_bytes!`.
- **Clippy and fmt must be clean** — PRs with lint warnings or format issues will not be merged.
- Keep changes focused and minimal. Don't refactor unrelated code.

## Documentation

The `docs/` directory is organized by purpose:

| Directory | Purpose |
|-----------|---------|
| `docs/workflow/` | Project architecture (`context.md`), current scope (`scope.md`), methodology (`workflow.md`) |
| `docs/plans/` | Implementation plans for each version |
| `docs/specs/` | Technical specifications for features |
| `docs/technical/` | Domain reference (known issues, deployment) |
| `docs/decisions.md` | Architecture Decision Records (ADRs) |

When making changes that affect behavior, update the relevant documentation.
