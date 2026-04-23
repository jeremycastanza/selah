# Technical Specification: Multi-Verse Bookmarks

_Created: 2026-04-15_
_Status: Draft_
_Author: AI-assisted_
_GitHub Issue: #4_

## Overview

Extend the bookmark system to support bookmarking a contiguous range of verses (e.g., John 3:16-18) rather than only a single verse at a time. This enables users to save passages, not just individual verses.

## Requirements

### Functional Requirements

1. **FR-1**: User can bookmark a range of verses within a single chapter
2. **FR-2**: Existing single-verse bookmarks continue to work (backward compatible)
3. **FR-3**: Bookmark list overlay displays ranges in natural format (e.g., "John 3:16-18")
4. **FR-4**: Selecting a range bookmark in the list navigates to the first verse of the range
5. **FR-5**: Snippet preview includes text from all verses in the range (truncated)
6. **FR-6**: Deduplication accounts for ranges — identical ranges are not added twice

### Non-Functional Requirements

1. **NFR-1**: Existing `bookmarks.json` files must be loadable without migration (backward compatible)
2. **NFR-2**: No `unsafe` Rust
3. **NFR-3**: Fully offline

## Technical Design

### Architecture

This is an extension of the existing `config/bookmarks.rs` module and its UI counterpart `ui/bookmarks.rs`. The `BookmarkEntry` struct gains an optional `verse_end` field.

### Data Model

```rust
// config/bookmarks.rs — modified
pub struct BookmarkEntry {
    pub book: String,
    pub chapter: u32,
    pub verse: u32,
    #[serde(default)]
    pub verse_end: Option<u32>,  // NEW — None means single verse
    pub snippet: Option<String>,
    pub note: Option<String>,
    pub created_at: u64,
}
```

`verse_end` is `Option<u32>` with `#[serde(default)]` so existing JSON files without this field deserialize cleanly as `None`.

### UX Flow

1. User navigates to a verse in the Verses panel and presses `b` — single-verse bookmark (existing behavior)
2. User presses `m` to enter **mark mode**: the current verse becomes the range start
3. User navigates to another verse in the same chapter and presses `m` again to set the range end
4. The range is bookmarked and a flash message confirms (e.g., "Bookmarked John 3:16-18")
5. Pressing `Esc` cancels mark mode

### Component Design

| Component | File | Responsibility |
|-----------|------|----------------|
| `BookmarkEntry` | `src/config/bookmarks.rs` | Extended data model with `verse_end` |
| `add()` / dedup logic | `src/config/bookmarks.rs` | Range-aware deduplication |
| `BrowserState` | `src/ui/browser.rs` | New `mark_start: Option<u32>` field for mark mode |
| `render_bookmarks` | `src/ui/bookmarks.rs` | Display range format in overlay |
| Key dispatch | `src/app.rs` | `m` key for mark mode |

### Keybindings

| Key | Context | Action |
|-----|---------|--------|
| `b` | Browser, verse selected | Single-verse bookmark (unchanged) |
| `m` | Browser, verse selected, no mark | Set range start (enter mark mode) |
| `m` | Browser, verse selected, mark active | Set range end, create bookmark |
| `Esc` | Browser, mark mode active | Cancel mark mode |

### Deduplication

A range bookmark `(book, chapter, verse, verse_end)` is considered a duplicate if an existing entry matches all four fields. Single-verse bookmarks (`verse_end: None`) use the existing `(book, chapter, verse)` check.

### Display Format

In the bookmark list overlay:
- Single verse: `John 3:16` (unchanged)
- Range: `John 3:16-18`

## Dependencies

No new dependencies required.

## Alternatives Considered

### Option A: Add `verse_end: Option<u32>` to existing struct

- **Pros**: Minimal change; backward compatible with `#[serde(default)]`
- **Cons**: Slightly overloads the single-verse bookmark concept

### Option B: Separate `BookmarkRange` type alongside `BookmarkEntry`

- **Pros**: Clean type separation
- **Cons**: Two bookmark types, two lists, more complex UI and persistence

### Decision

Option A — extend the existing struct. The `Option<u32>` is the simplest change with full backward compatibility. A range is just a bookmark where `verse_end.is_some()`.

## Security Considerations

- No new attack surface — same local JSON persistence
- Range validation: `verse_end` must be >= `verse` and within chapter bounds

## Testing Strategy

- Unit tests: `config/bookmarks.rs` — add range bookmark, dedup ranges, serde round-trip with and without `verse_end`, backward compat with old JSON
- Unit tests: Mark mode state transitions in `BrowserState`
- Integration tests: Create range bookmark, view in overlay, navigate to it
- Manual verification: Mark mode visual indicator, range display in bookmark list

## Resolved Questions

- **Mark mode indicator location?** — Verses panel. The selected mark-start verse should be visually indicated in the Verses panel (e.g., highlighted or marked).
- **Cross-chapter ranges?** — No. Ranges are strictly within a single chapter for v0.2.0.

## References

- [Current bookmark implementation](../../src/config/bookmarks.rs)
- [Bookmark overlay](../../src/ui/bookmarks.rs)
- [Bookmark key handling](../../src/app.rs:293) — current `b` key handler
- GitHub Issue: #4
