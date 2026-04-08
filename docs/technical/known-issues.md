# Known Issues

_Last updated: 2026-04-08_

## Overview

No active issues. All unit tests pass. `cargo clippy -- -D warnings` and `cargo fmt --check` are clean. v0.1.0 released.

## Active Issues

None.

## Limitations

| Area | Limitation | Impact | Planned Fix? |
|------|-----------|--------|--------------|
| Bible versions | Only KJV is bundled | Users cannot switch translations | Future version |
| FTS5 index | Built at runtime, not persisted | ~100ms startup cost on first search | No (acceptable) |
| Build targets | ARM only (aarch64) | x86 users must build from source | Possible future CI addition |

## Environment-Specific Issues

None known.

## Resolved Issues

| Area | Issue | Resolution |
|------|-------|------------|
| Bookmarks | Stub only; not functional | Implemented in Phase 7 |
| Splash screen | Placeholder text | Full animation implemented in Phase 8 |
| CI/CD | No automated release pipeline | cargo-dist pipeline implemented in Phase 9 |

## Related Documents

- `docs/architecture.md` — System design
- `docs/decisions.md` — Architecture Decision Records
