# Integrations

_Last updated: 2026-04-01_

## Overview

Selah has no external service integrations. It is a fully offline application — all data is bundled with the binary and there are no runtime network dependencies.

## Embedded Data

### KJV Bible Database

**Source:** [scrollmapper/bible_databases](https://github.com/scrollmapper/bible_databases)

**Format:** SQLite (`.sqlite`)

**How it's used:** `data/kjv.sqlite` is embedded at compile time via `include_bytes!` in `src/bible/db.rs`. On startup, `open_db()` writes the bytes to a temp file and opens a `rusqlite::Connection`. The FTS5 full-text index is built from the verses table at runtime.

**No API key, no network request, no auth required.**

## Related Documents

- `docs/architecture.md` — System design
- `docs/technical/known-issues.md` — Known issues
