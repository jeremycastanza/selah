# Deployment

_Last updated: 2026-05-04_

## Distribution Model

Selah is distributed as a compiled Rust binary via:

1. **Homebrew tap** — `jeremycastanza/homebrew-selah` (public)
2. **Shell installer** — `curl | sh` from GitHub Releases
3. **GitHub Releases** — direct tarball download

## CI/CD Pipeline

Powered by [cargo-dist](https://github.com/axodotdev/cargo-dist) v0.31.0.

**Trigger:** Push a tag matching `v*.*.*` (e.g., `v1.0.0`).

**Config:** `dist-workspace.toml`

### Pipeline Stages

1. **Plan** — `dist plan` determines build matrix
2. **Build local artifacts** — Compile binaries for each target
3. **Build global artifacts** — Generate shell installer, checksums, Homebrew formula
4. **Host** — Upload artifacts and create GitHub Release
5. **Publish Homebrew** — Push the cargo-dist generated formula (`selah.rb`) to the tap repo
6. **Announce** — Finalize release

### Build Targets

| Target | Platform |
|--------|----------|
| `aarch64-apple-darwin` | macOS ARM64 |
| `aarch64-unknown-linux-gnu` | Linux ARM64 |

### Artifacts per Release

- `selah-aarch64-apple-darwin.tar.xz` + `.sha256`
- `selah-aarch64-unknown-linux-gnu.tar.xz` + `.sha256`
- `selah-installer.sh` (shell installer)
- `selah.rb` (Homebrew formula)
- `sha256.sum`
- `dist-manifest.json`

## Homebrew Tap

| Item | Details |
|------|---------|
| Tap repo | [`jeremycastanza/homebrew-selah`](https://github.com/jeremycastanza/homebrew-selah) (public) |
| Tap command | `brew tap jeremycastanza/selah` |
| Install | `brew install selah` |
| Update | `brew upgrade selah` |

The formula uses direct public release download URLs from the `selah` repo — no authentication required for users.

### How the Tap is Updated

The `publish-homebrew` CI job:
1. Downloads the cargo-dist generated `selah.rb` from build artifacts
2. Pushes it to `Formula/selah.rb` in the tap repo via the GitHub API

The tap repo contains **only** the formula and a README — no releases, no binary assets. All binaries are downloaded from the `selah` repo's GitHub Releases.

Requires a `HOMEBREW_TAP_TOKEN` secret in the `selah` repo (fine-grained PAT with `contents:write` on `homebrew-selah`).

### Tap Branch Ruleset

The tap has a branch ruleset on `main` with a bypass for the Repository Admin role. This allows the CI job (authenticated via `HOMEBREW_TAP_TOKEN`) to push formula updates directly without a PR.

## Release Process

```bash
# 1. Bump version in Cargo.toml
# 2. Commit and merge to main
# 3. Tag and push (annotated tags only)
git tag -a v<version> -m "v<version>"
git push origin v<version>

# CI handles everything from here:
# - Builds binaries for all targets
# - Creates GitHub Release with artifacts
# - Pushes updated formula to Homebrew tap
```

## Related Documents

- `docs/architecture.md` — System design
- `.github/workflows/release.yml` — CI workflow
- `dist-workspace.toml` — cargo-dist configuration
