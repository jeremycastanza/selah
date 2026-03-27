# Technical Specification: F7 — Search

_Created: 2026-03-26_
_Status: Draft_

## Overview

Full-text search across all verses in the current translation using SQLite FTS5. Results appear live as the user types. Selecting a result navigates to that verse.

## Requirements

### Functional

- **FR-1**: Press `/` in browser mode to open the search overlay
- **FR-2**: Results update live after each keystroke (minimum 3 characters to trigger)
- **FR-3**: Results are ranked by BM25 relevance (SQLite FTS5 default ranking)
- **FR-4**: Result list shows up to 50 results; each entry shows `Book Ch:V — [verse text snippet]`
- **FR-5**: `↑/↓` navigates the result list
- **FR-6**: `Enter` on a selected result closes the search overlay and navigates to that verse
- **FR-7**: `Esc` closes the search overlay without navigating
- **FR-8**: Search query and results are cleared when the overlay is closed

### Non-Functional

- **NFR-1**: Search must work fully offline — no network requests
- **NFR-2**: SQLite FTS5 query must complete in < 50ms (in-memory database)
- **NFR-3**: Minimum query length of 3 characters prevents excessive results

## User Interaction Flow

1. User presses `/` — search overlay opens at bottom of screen; cursor-like input field is active
2. User types characters — results appear live below the input field (after 3+ characters)
3. User presses `↑/↓` to select a result
4. User presses `Enter` — overlay closes; browser navigates to `Book Ch:V`; chapter loads; Scripture scrolls to that verse
5. Or user presses `Esc` — overlay closes; browser returns to previous state unchanged

## Technical Design

### Search State

`SearchMode` enum in `ui/browser.rs` (or dedicated `ui/search.rs`):

```rust
pub enum SearchMode {
    Off,
    Active {
        query: String,
        results: Vec<SearchResult>,
        list_state: ListState,
    },
}
```

This is held in `BrowserState.overlay` as `OverlayKind::Search(SearchState)`.

### SQLite FTS5 Query

```rust
// bible/db.rs
pub fn search(conn: &Connection, query: &str, translation: &str) -> rusqlite::Result<Vec<SearchResult>> {
    let table = format!("t_{}_fts", translation.to_lowercase());
    let sql = format!(
        "SELECT b, c, v, t FROM {} WHERE {} MATCH ?1 ORDER BY rank LIMIT 50",
        table, table
    );
    // map rows to SearchResult
}
```

The FTS5 `MATCH` operator handles phrase search, prefix search, and boolean operators natively. No custom tokenization needed.

### Search Result Type

```rust
// bible/types.rs
pub struct SearchResult {
    pub book: String,
    pub book_num: u32,
    pub chapter: u32,
    pub verse: u32,
    pub text: String,
}
```

Book name is resolved from `book_num` via `BOOKS[book_num - 1].name`.

### Overlay Layout

The search overlay renders as a full-width panel at the bottom portion of the terminal (approximately 40% height). It uses `Clear` to blank the region first, then renders:

```
┌─ Search ────────────────────────────────────────────────────────┐
│ > faith hope love_                                              │
├─────────────────────────────────────────────────────────────────┤
│  ► 1 Corinthians 13:13 — And now these three remain: faith...   │
│    Hebrews 11:1 — Now faith is the substance of things...       │
│    Romans 5:1 — Therefore, since we have been justified...      │
│    ...                                                          │
└─────────────────────────────────────────────────────────────────┘
```

The input line renders the query string with a blinking cursor approximation (appended `_` or `|`).

### Live Search Trigger

Search fires on every character change when `query.len() >= 3`. When `query.len() < 3`, results are cleared.

Since SQLite is synchronous and in-memory, search runs on the main thread. No async/thread spawning needed.

### Key Handling in Search Mode

The key handler checks `SearchMode` first before normal browser key dispatch:

```rust
if let Some(OverlayKind::Search(ref mut s)) = state.overlay {
    match key {
        KeyCode::Esc      => state.overlay = None,
        KeyCode::Backspace => { s.query.pop(); run_search(); }
        KeyCode::Char(c)   => { s.query.push(c); run_search(); }
        KeyCode::Up        => { /* move list_state selection up */ }
        KeyCode::Down      => { /* move list_state selection down */ }
        KeyCode::Enter     => {
            if let Some(result) = s.selected_result() {
                state.jump_to_verse(result.book_num, result.chapter, result.verse);
                state.overlay = None;
                load_chapter();
            }
        }
        _ => {}
    }
    return;
}
```

## Dependencies

- F1 (overlay renders on top of browser layout; `jump_to_verse` and chapter load reuse browser state methods)
- `bible/db.rs` — FTS5 search query
- Data Layer — FTS5 virtual table must exist for the active translation

## Open Questions

- **FTS5 table initialization:** The `scrollmapper/bible_databases` SQLite file may or may not include FTS5 tables pre-built. If not, a one-time `INSERT INTO t_kjv_fts(t_kjv_fts) VALUES ('rebuild')` rebuild is needed on first run, or the FTS5 table must be created as part of the database build pipeline. Needs verification against the actual source database file.

## Testing Strategy

- **Manual**: Type `"love"` — verify results appear; press `Enter` on first result — verify navigation
- **Manual**: Type `"ab"` (2 chars) — verify no results appear
- **Manual**: Type `"hope faith love"` — verify multi-word phrase search returns relevant results
- **Manual**: Press `Esc` — verify search closes and browser state is unchanged
- **Automated**: Unit test `search()` with known KJV phrase returns expected book/chapter/verse
- **Automated**: Unit test that `search()` returns empty for a query with no matches
- **Automated**: Unit test minimum-length guard (< 3 chars returns empty vec)

## References

- `christ-cli` source: `src/app.rs` (`SearchMode` enum and key dispatch), `src/data/kjv.rs` (offline search reference)
- SQLite FTS5 documentation: https://www.sqlite.org/fts5.html
