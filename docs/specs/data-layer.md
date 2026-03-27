# Technical Specification: Data Layer

_Created: 2026-03-26_
_Status: Draft_

## Overview

All Bible text is bundled inside the binary. There are no runtime network requests. Session state and bookmarks are persisted to JSON files in the platform's standard data directory.

## Bible Text Storage

### Embedding Strategy — Two Databases

Bible data is split across two SQLite databases:

1. **`data/kjv.sqlite`** (~1.2 MB) — KJV only. Always bundled. Committed directly to git (small enough that LFS is unnecessary). This is the base database and guarantees the app always works offline out of the box.

2. **`data/extra.sqlite`** (variable, potentially 30+ MB) — All other translations. **Not committed to git.** Either:
   - Tracked via Git LFS, or
   - Generated at build time via `build.rs` from `scrollmapper/bible_databases` SQL sources

Both are embedded at compile time using `include_bytes!()`:

```rust
// bible/db.rs

// Always available
const KJV_DB: &[u8] = include_bytes!("../../data/kjv.sqlite");

// Optional — only present when built with extra translations
#[cfg(feature = "extra-translations")]
const EXTRA_DB: &[u8] = include_bytes!("../../data/extra.sqlite");
```

On startup, the KJV database is always opened in-memory. The extra database is opened only if the feature is enabled and the bytes are present. If a user selects a translation that isn't available, the version picker shows only KJV (graceful no-op per F8 spec).

A plain `cargo build` always succeeds with KJV only. `cargo build --features extra-translations` bundles the additional translations.

### Data Source

`scrollmapper/bible_databases` (MIT licensed). The repository provides pre-built `.sqlite` files for 140+ translations. v1 bundles KJV only (base database). Additional translations are added to the extra database once the build pipeline is validated.

### Database Schema

Tables are sourced from `scrollmapper/bible_databases` and used as-is:

```sql
-- Bible text (one table per translation, e.g. t_kjv, t_web, t_esv)
CREATE TABLE t_kjv (
    b   INTEGER NOT NULL,  -- book number (1–66, 1=Genesis)
    c   INTEGER NOT NULL,  -- chapter number (1-based)
    v   INTEGER NOT NULL,  -- verse number (1-based)
    t   TEXT    NOT NULL   -- verse text
);

-- Full-text search virtual table (FTS5) per translation
CREATE VIRTUAL TABLE t_kjv_fts USING fts5(
    t,
    content='t_kjv',
    content_rowid='rowid'
);

-- Book abbreviations (used for CLI reference parsing)
CREATE TABLE key_abbreviations_english (
    id  INTEGER PRIMARY KEY,
    a   TEXT NOT NULL,  -- abbreviation string
    b   INTEGER NOT NULL  -- book number
);
```

### Translation Table Naming

Each translation uses the pattern `t_{code_lowercase}`, e.g.:
- `t_kjv` — King James Version
- `t_web` — World English Bible
- `t_esv` — English Standard Version

Query functions accept a translation code string and compute the table name:

```rust
fn verse_table(translation: &str) -> String {
    format!("t_{}", translation.to_lowercase())
}
```

### Book Number Resolution

Book numbers (1–66) are resolved to names via the static `BOOKS` array in `bible/books.rs`. Indexed as `BOOKS[b as usize - 1]`. Ported directly from `christ-cli`'s `data/books.rs`.

## Core Rust Types

```rust
// bible/types.rs

pub struct Verse {
    pub book: String,
    pub book_num: u32,
    pub chapter: u32,
    pub verse: u32,
    pub text: String,
    pub translation: String,
}

pub struct Chapter {
    pub book: String,
    pub chapter: u32,
    pub verses: Vec<Verse>,
}

pub struct SearchResult {
    pub book: String,
    pub book_num: u32,
    pub chapter: u32,
    pub verse: u32,
    pub text: String,
}
```

## Query Patterns

All queries are synchronous (no async/tokio). SQLite in-memory access is sub-millisecond.

```rust
// Fetch a full chapter
SELECT b, c, v, t FROM t_kjv WHERE b = ? AND c = ? ORDER BY v

// Fetch a single verse
SELECT b, c, v, t FROM t_kjv WHERE b = ? AND c = ? AND v = ?

// FTS5 search (max 50 results, BM25 ranked)
SELECT b, c, v, t FROM t_kjv_fts WHERE t_kjv_fts MATCH ? ORDER BY rank LIMIT 50

// Random verse
SELECT b, c, v, t FROM t_kjv ORDER BY RANDOM() LIMIT 1
```

## Session Persistence

### Location

| Platform | Path |
|---|---|
| macOS | `~/Library/Application Support/selah/session.json` |
| Linux | `~/.local/share/selah/session.json` |

Resolved via the `directories` crate: `ProjectDirs::from("", "", "selah")?.data_dir()`.

### Schema

```rust
// config/session.rs
#[derive(Serialize, Deserialize, Default)]
pub struct SessionState {
    pub book_index: usize,
    pub chapter: u32,
    pub scroll_position: u16,
    pub active_panel: u8,    // 0=Books, 1=Chapters, 2=Scripture
    pub theme: ThemeName,
    pub translation: String,
}
```

```json
{
  "book_index": 42,
  "chapter": 3,
  "scroll_position": 0,
  "active_panel": 2,
  "theme": "Slate",
  "translation": "KJV"
}
```

Session is loaded on startup and saved on clean quit.

## Bookmark Persistence

### Location

| Platform | Path |
|---|---|
| macOS | `~/Library/Application Support/selah/bookmarks.json` |
| Linux | `~/.local/share/selah/bookmarks.json` |

### Schema

```rust
// config/bookmarks.rs
#[derive(Serialize, Deserialize)]
pub struct BookmarkEntry {
    pub book: String,
    pub chapter: u32,
    pub verse: u32,
    pub note: Option<String>,
    pub created_at: u64,  // Unix timestamp (seconds)
}
```

```json
[
  {
    "book": "John",
    "chapter": 3,
    "verse": 16,
    "note": "For God so loved...",
    "created_at": 1711411200
  }
]
```

The full bookmark list is loaded into `App.bookmarks: Vec<BookmarkEntry>` at startup. Mutations (add/remove) flush the entire list to disk immediately. Empty file is `[]`.

## Resolved Decisions

1. **Two-database strategy** — KJV base database committed directly to git (~1.2 MB). Extra translations in a separate database, excluded from git and either LFS'd or generated via build.rs. This keeps `cargo build` fast and the repo small while allowing opt-in multi-translation builds.

2. **Database file in git** — `data/kjv.sqlite` committed directly (small enough). `data/extra.sqlite` excluded via `.gitignore`.

## Open Questions

1. **Extra translations: LFS vs. build.rs** — For the extra database, should it be tracked via Git LFS (simpler, but requires LFS setup for contributors) or generated at build time from scrollmapper SQL sources (more complex, but fully reproducible)? Decision needed before adding second translation.

2. **In-memory budget for extra DB** — If `extra.sqlite` grows past ~30 MB, loading it entirely in-memory may be excessive. At that point, extracting to a platform cache dir on first run may be preferable. Monitor binary size as translations are added.

## References

- [Architecture spec](architecture.md)
- `scrollmapper/bible_databases`: https://github.com/scrollmapper/bible_databases
- `christ-cli` source: `src/data/kjv.rs`, `src/data/books.rs`, `src/store/cache.rs`
