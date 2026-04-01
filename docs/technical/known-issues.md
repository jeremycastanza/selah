# Known Issues

_Last updated: 2026-04-01_

## Overview

No active issues at this time. All 28 unit tests pass. `cargo clippy -- -D warnings` and `cargo fmt --check` are clean.

## Active Issues

None.

## Limitations

| Area | Limitation | Impact | Planned Fix? |
|------|-----------|--------|--------------|
| Bible version | Only KJV is bundled | Users cannot switch translations | Yes — Phase 7 |
| Bookmarks | Stub only; not functional | Users cannot save verses | Yes — Phase 7 |
| Splash screen | Banner is placeholder text | No visual splash on startup | Yes — Phase 8 |
| FTS5 index | Built at runtime, not persisted | ~100ms startup cost on first search | No (acceptable) |

## Environment-Specific Issues

None known.

## Resolved Issues

None yet.

## Related Documents

- `docs/architecture.md` — System design
- `docs/decisions.md` — Architecture Decision Records
