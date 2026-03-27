# Technical Specification: F4 — Bookmark System

_Created: 2026-03-26_
_Status: Draft_

## Overview

Users can bookmark any verse with an optional note. Bookmarks persist across sessions in a JSON file. A bookmark overlay lists all saved bookmarks and allows navigation to them or deletion.

## Requirements

### Functional

- **FR-1**: Press `b` in browser mode to bookmark the currently focused verse
- **FR-2**: A brief status bar flash confirms the bookmark was added (e.g., "Bookmarked John 3:16")
- **FR-3**: Press `B` (shift+b) to open the bookmark list overlay
- **FR-4**: In the bookmark overlay, `j/k` navigates the list; `Enter` jumps to that verse; `d` deletes the selected bookmark; `Esc` closes without action
- **FR-5**: Mouse click on a bookmark item navigates to that verse
- **FR-6**: Bookmarks persist to disk and survive restarts
- **FR-7**: Bookmarking a verse that is already bookmarked is a no-op (no duplicate entries)

### Non-Functional

- **NFR-1**: Bookmark list is loaded into memory at startup; mutations flush to disk immediately
- **NFR-2**: Bookmark file format is human-readable JSON

## User Interaction Flow

**Adding a bookmark:**
1. User navigates to a verse (Verses panel focused or Scripture showing)
2. User presses `b`
3. Status bar shows `"Bookmarked John 3:16"` for 2 seconds
4. Bookmark is persisted to `bookmarks.json`

**Viewing and navigating bookmarks:**
1. User presses `B` — bookmark overlay appears (centered modal)
2. List shows all bookmarks: `John 3:16 — For God so loved...` (truncated)
3. User presses `j/k` to navigate
4. User presses `Enter` — overlay closes, browser navigates to that verse
5. Or user presses `d` — selected bookmark is removed, list updates
6. Or user presses `Esc` — overlay closes, no change

## Technical Design

### Data Model

```rust
// config/bookmarks.rs
#[derive(Serialize, Deserialize)]
pub struct BookmarkEntry {
    pub book: String,
    pub chapter: u32,
    pub verse: u32,
    pub note: Option<String>,
    pub created_at: u64,  // Unix timestamp (seconds via std::time::SystemTime)
}
```

File location:
- macOS: `~/Library/Application Support/selah/bookmarks.json`
- Linux: `~/.local/share/selah/bookmarks.json`

### Storage Functions

```rust
// config/bookmarks.rs
pub fn load() -> Vec<BookmarkEntry>
pub fn save(bookmarks: &[BookmarkEntry])
pub fn add(bookmarks: &mut Vec<BookmarkEntry>, entry: BookmarkEntry)
pub fn remove(bookmarks: &mut Vec<BookmarkEntry>, index: usize)
```

`add()` checks for duplicates by `(book, chapter, verse)` before inserting. `save()` writes the entire list as pretty-printed JSON.

### App State

`App` holds the bookmark list in memory:

```rust
pub struct App {
    // ...
    pub bookmarks: Vec<BookmarkEntry>,
}
```

### Overlay

`BrowserState.overlay = Some(OverlayKind::Bookmarks(BookmarkListState))` opens the overlay.

```rust
pub struct BookmarkListState {
    pub list_state: ListState,
}
```

`ui/bookmarks.rs` renders a centered `Clear` + `Block` with a `List` of bookmark entries. Each item displays: `"Book Ch:V — [first 40 chars of verse text]"`.

To show verse preview text in the overlay, each `BookmarkEntry` needs the verse text at render time. Two options:
1. Store verse text in the bookmark entry (denormalized, but avoids a query per render)
2. Query SQLite for each visible item on render (clean, but adds overhead)

**Recommendation for v1:** Store a `snippet: Option<String>` (first 60 chars of verse text) in `BookmarkEntry`, populated when the bookmark is added. This avoids per-render queries.

### Status Flash

`BrowserState` holds:

```rust
pub status_flash: Option<(String, Instant)>,
```

Set on bookmark add: `state.status_flash = Some(("Bookmarked John 3:16".into(), Instant::now()))`. Cleared during render if elapsed > 2 seconds.

## Dependencies

- F1 (overlay renders on top of browser layout; `status_flash` displayed in status bar)
- F3 (mouse click on bookmark list item navigates to verse)
- `config/bookmarks.rs` — persistence
- `bible/db.rs` — verse text lookup when adding a bookmark (for the `snippet` field)

## Testing Strategy

- **Manual**: Press `b` on a verse — confirm status bar flash appears
- **Manual**: Press `B` — confirm overlay shows the added bookmark
- **Manual**: Navigate to bookmark with `Enter` — confirm browser jumps to correct verse
- **Manual**: Delete a bookmark with `d` — confirm it disappears from the list and is gone after restart
- **Manual**: Add the same verse twice — confirm only one entry appears
- **Manual**: Quit and relaunch — confirm bookmarks persist
- **Automated**: Unit test `add()` deduplication logic
- **Automated**: Unit test `save()` / `load()` round-trip with a `Vec<BookmarkEntry>`

## References

- `bible-tui` source: `utils/config.js` (bookmark schema reference — `{ book, chapter, verse, note }`)
- `bible-tui` source: `cli/index.js` (`bookmark` and `bookmarks` commands — UX reference)
