# Deployment

_Last updated: 2026-03-25_

## Distribution Model

Selah is distributed as a compiled Rust binary via a **private Homebrew tap**, authenticated using the GitHub CLI (`gh`). This mirrors the lexicon-dex CLI deployment pattern.

## Homebrew Tap

| Item | Details |
|------|---------|
| Tap type | Private GitHub repo |
| Auth | GitHub CLI (`gh auth`) |
| Install command | `brew install <org>/<tap>/selah` |

## Build & Release Process

### Prerequisites

- [ ] Rust toolchain installed (`rustup`)
- [ ] `gh` CLI authenticated
- [ ] Homebrew tap repo access

### Build

```bash
cargo build --release
```

### Release

```bash
# Tag a new release
git tag -a v<version> -m "v<version>"
git push origin v<version>

# GitHub Actions publishes the binary and updates the tap formula
```

## CI/CD Pipeline

TBD — likely GitHub Actions for cross-platform builds (macOS, Linux) and automated tap formula updates.

### Pipeline Stages

1. **Build** — `cargo build --release` for target platforms
2. **Test** — `cargo test`
3. **Release** — Upload binary to GitHub Release, update Homebrew formula

## Related Documents

- `docs/architecture.md` — System design
