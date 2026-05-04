# Known Issues

_Last updated: 2026-05-04_

## Overview

No active issues. `cargo clippy -- -D warnings` and `cargo fmt --check` are clean. v0.4.0 released.

## Active Issues

- Homebrew tap still uses PAT-authenticated URLs (private repo). Must rewrite formula before public.

## Limitations

| Area | Limitation | Impact | Planned Fix? |
|------|-----------|--------|--------------|
| Bible versions | Additional translations require API key | Users without YVP key get KJV only | No (by design) |
| FTS5 index | Built at runtime, not persisted | ~100ms startup cost on first search | No (acceptable) |
| Build targets | ARM only (aarch64) | x86 users must build from source | Possible future CI addition |
| Homebrew | Tap is private; requires PAT | Users can't `brew install` without auth | In progress (tap public prep) |

## Environment-Specific Issues

None known.

## Resolved Issues

| Area | Issue | Resolution |
|------|-------|------------|
| Bookmarks | Stub only; not functional | Implemented in v0.1.0 |
| Splash screen | Placeholder text | Full animation in v0.1.0 |
| CI/CD | No automated release pipeline | cargo-dist in v0.1.0 |
| Translations | Only KJV bundled | API integration in v0.3.0 |
| Help/bindings | No discoverable keybinding reference | Menu overlay in v0.4.0 |

## Related Documents

- `docs/architecture.md` — System design
- `docs/decisions.md` — Architecture Decision Records
