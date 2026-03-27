# Technical Specification: Build and Distribution

_Created: 2026-03-26_
_Status: Draft_

## Overview

Selah compiles to a single static binary with all Bible data embedded. It is distributed via a private Homebrew tap on GitHub.

## Cargo Configuration

Single-crate workspace — no workspace setup needed. One `Cargo.toml`, one `[[bin]]`:

```toml
[package]
name = "selah"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "selah"
path = "src/main.rs"

[dependencies]
ratatui    = { version = "0.29", features = ["crossterm"] }
crossterm  = "0.28"
rusqlite   = { version = "0.32", features = ["bundled"] }
serde      = { version = "1", features = ["derive"] }
serde_json = "1"
clap       = { version = "4", features = ["derive"] }
directories = "6"
rand       = "0.9"
unicode-width = "0.2"

[features]
extra-translations = []  # Bundle additional Bible translations beyond KJV

[profile.release]
opt-level = "z"
lto       = true
strip     = true
```

The `rusqlite` `bundled` feature compiles SQLite from C source at build time — no system SQLite required.

## Build Targets

| Target | Platform |
|---|---|
| `aarch64-apple-darwin` | macOS (Apple Silicon) |
| `x86_64-apple-darwin` | macOS (Intel) |
| `x86_64-unknown-linux-gnu` | Linux (x86_64) |
| `aarch64-unknown-linux-gnu` | Linux (ARM64) |

Windows is not a supported target.

## Data Embedding

Two SQLite databases, embedded at compile time:

```rust
// bible/db.rs

// Always bundled — KJV only (~4.5 MB, FTS5 index built at runtime)
const KJV_DB: &[u8] = include_bytes!("../../data/kjv.sqlite");

// Optional — extra translations, gated behind a feature flag
#[cfg(feature = "extra-translations")]
const EXTRA_DB: &[u8] = include_bytes!("../../data/extra.sqlite");
```

- `data/kjv.sqlite` — committed to git (~4.5 MB). Sourced from `scrollmapper/bible_databases`. FTS5 index excluded from on-disk DB and built at runtime.
- `data/extra.sqlite` — **not committed to git** (`.gitignore`'d). Either LFS'd or generated via build.rs. Only included when building with `--features extra-translations`.

## Binary Size Estimates

| Component | Estimated Size |
|---|---|
| Rust binary (stripped, release, LTO) | ~2–3 MB |
| rusqlite bundled SQLite | ~1.5 MB |
| KJV database (embedded) | ~4.5 MB |
| **Total (v1, KJV only)** | **~8–9 MB** |

With extra translations enabled, binary grows by ~1–3 MB per additional translation in `extra.sqlite`.

## CI / GitHub Actions

Build workflow triggers on tag push (`v*`):

1. `cargo build --release` for `aarch64-apple-darwin` and `x86_64-apple-darwin` on macOS runner
2. `cross build --release --target x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu` on Linux runner via `cross` (Docker-based cross-compilation)
3. Upload binaries to GitHub Release
4. Update Homebrew formula with new version and SHA256 checksums

## Distribution

Private Homebrew tap on GitHub. Users install with:

```bash
brew tap <owner>/selah https://github.com/<owner>/homebrew-selah
brew install selah
```

Formula template:
```ruby
class Selah < Formula
  desc "A terminal Bible reader"
  homepage "https://github.com/<owner>/selah"
  version "0.1.0"

  on_macos do
    on_arm do
      url "..."
      sha256 "..."
    end
    on_intel do
      url "..."
      sha256 "..."
    end
  end

  on_linux do
    on_arm do
      url "..."
      sha256 "..."
    end
    on_intel do
      url "..."
      sha256 "..."
    end
  end

  def install
    bin.install "selah"
  end
end
```

## Local Development

```bash
# Build and run (debug)
cargo run

# Build and run (release)
cargo build --release && ./target/release/selah

# Run tests
cargo test

# Check for issues
cargo clippy -- -D warnings
cargo fmt --check
```

## Resolved Decisions

1. **Two-database strategy** — `data/kjv.sqlite` committed directly to git. `data/extra.sqlite` excluded from git, gated behind `extra-translations` feature flag. See [Data Layer spec](data-layer.md) for details.

## Open Questions

1. **`cross` for Linux ARM** — Linux ARM64 builds require `cross` or a native ARM runner. Confirm CI runner availability before setting up the release workflow.

## References

- ADR-003: Private Homebrew Tap for Distribution
- `christ-cli` `Cargo.toml` (build profile reference)
- `scrollmapper/bible_databases`: https://github.com/scrollmapper/bible_databases
