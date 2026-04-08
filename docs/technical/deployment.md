# Deployment

_Last updated: 2026-04-08_

## Distribution Model

Selah is distributed as a compiled Rust binary via:

1. **Private Homebrew tap** — `jeremycastanza/homebrew-selah-tap`
2. **Shell installer** — `curl | sh` from GitHub Releases
3. **GitHub Releases** — direct tarball download

## CI/CD Pipeline

Powered by [cargo-dist](https://github.com/axodotdev/cargo-dist) v0.31.0.

**Trigger:** Push a tag matching `v*.*.*` (e.g., `v0.1.0`).

**Config:** `dist-workspace.toml`

### Pipeline Stages

1. **Plan** — `dist plan` determines build matrix
2. **Build local artifacts** — Compile binaries for each target
3. **Build global artifacts** — Generate shell installer, checksums, Homebrew formula
4. **Host** — Upload artifacts and create GitHub Release
5. **Publish tap** — Mirror release assets to `homebrew-selah-tap` and update the formula
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
| Tap repo | `jeremycastanza/homebrew-selah-tap` (private) |
| Auth | `HOMEBREW_GITHUB_API_TOKEN` env var (GitHub PAT with `repo` scope) |
| Tap command | `brew tap jeremycastanza/selah-tap git@github-personal:jeremycastanza/homebrew-selah-tap.git` |
| Install | `brew install selah` |
| Update | `brew upgrade selah` |

The formula downloads pre-built binaries from the tap repo's own releases (not the source repo), using GitHub API asset URLs with token-based auth headers. This mirrors the `homebrew-lexicon-tap` pattern.

### Tap Release Automation

The `publish-tap` CI job:
1. Downloads build artifacts from the main release
2. Creates a matching release on the tap repo with binary assets
3. Generates a Homebrew formula (`Formula/selah.rb`) with correct asset IDs and SHA256 checksums
4. Pushes the formula to the tap repo via the GitHub API

Requires a `HOMEBREW_TAP_TOKEN` secret in the source repo with write access to the tap repo.

## Release Process

```bash
# Tag a new release (annotated)
git tag -a v<version> -m "v<version>"
git push origin v<version>

# CI handles everything from here:
# - Builds binaries for all targets
# - Creates GitHub Release with artifacts
# - Publishes to Homebrew tap
```

## Related Documents

- `docs/architecture.md` — System design
- `.github/workflows/release.yml` — CI workflow
- `dist-workspace.toml` — cargo-dist configuration
