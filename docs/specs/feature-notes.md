# Technical Specification: Verse Notes

_Created: 2026-04-15_
_Status: Draft_
_Author: AI-assisted_
_GitHub Issue: #2_

## Overview

Add the ability for users to attach free-text notes to individual verses. Notes are persisted locally alongside bookmarks and restored across sessions, enabling users to record study insights without leaving the terminal.

## Requirements

### Functional Requirements

1. **FR-1**: User can add a note to any verse currently displayed in the Scripture panel
2. **FR-2**: User can view existing notes for the selected verse
3. **FR-3**: User can edit an existing note (replace content)
4. **FR-4**: User can delete a note from a verse
5. **FR-5**: Notes are persisted to disk and survive app restart
6. **FR-6**: Notes are displayed inline or via overlay when viewing a verse that has one

### Non-Functional Requirements

1. **NFR-1**: Notes must work fully offline — no network dependencies
2. **NFR-2**: Note storage must not degrade startup time perceptibly (< 50ms for 1000 notes)
3. **NFR-3**: No `unsafe` Rust

## Technical Design

### Architecture

Notes live in the `config/` layer alongside bookmarks and session state. A new `config/notes.rs` module handles persistence. The UI surfaces notes through a new `OverlayKind::Notes` variant — a text-input overlay triggered by a keybinding.

### Data Model

```rust
// config/notes.rs
pub struct NoteEntry {
    pub book: String,
    pub chapter: u32,
    pub verse: u32,
    pub text: String,
    pub created_at: u64,
    pub updated_at: u64,
}
```

Stored as `notes.json` in the platform data directory (`directories` crate), same location as `bookmarks.json` and `session.json`.

### Key Identity

A note is uniquely identified by `(book, chapter, verse)`. One note per verse. If a user wants multiple thoughts, they write them in the same note body.

### Component Design

| Component | File | Responsibility |
|-----------|------|----------------|
| `NoteEntry` | `src/config/notes.rs` | Data model + load/save/add/update/remove |
| `NoteEditorState` | `src/ui/notes.rs` | Overlay state: text buffer, cursor position |
| `OverlayKind::Notes` | `src/ui/browser.rs` | New overlay variant |
| `render_notes` | `src/ui/notes.rs` | Overlay rendering |
| Key dispatch | `src/app.rs` | `n` to open note editor, note indicator in scripture |

### Keybindings

| Key | Context | Action |
|-----|---------|--------|
| `n` | Browser, verse selected | Open note editor for selected verse |
| `Enter` | Note editor | Save note and close overlay |
| `Esc` | Note editor | Discard changes and close overlay |

### Scripture Panel Integration

When rendering verses in the Scripture panel, check if a note exists for each verse. If so, append a visual indicator (e.g., `[*]` or a dim note icon) after the verse number.

## Dependencies

No new dependencies required. Uses existing `serde`, `serde_json`, `directories`.

## Alternatives Considered

### Option A: Embed notes inside BookmarkEntry

Extend the existing `BookmarkEntry.note` field (already present but unused in practice).

- **Pros**: No new file, reuses existing persistence
- **Cons**: Couples notes to bookmarks — user must bookmark a verse to annotate it; doesn't match user story

### Option B: Separate notes.json file

Dedicated `NoteEntry` type and `notes.json` file.

- **Pros**: Notes are independent of bookmarks; cleaner separation of concerns
- **Cons**: One more file to manage

### Decision

Option B — separate `notes.json`. Notes should exist independently of bookmarks. The existing `BookmarkEntry.note` field can remain for brief bookmark labels.

## Security Considerations

- Notes are stored as plain JSON on the local filesystem — no encryption needed for v0.2.0
- No user input is sent over the network
- Text input in the overlay should not allow control character injection into the terminal

## Testing Strategy

- Unit tests: `config/notes.rs` — add, update, remove, dedup by verse identity, serde round-trip
- Unit tests: `NoteEditorState` — text buffer manipulation, cursor movement
- Integration tests: Open note overlay, type text, save, verify persistence
- Manual verification: Note indicator visible in scripture panel; notes survive restart

## Resolved Questions

- **Multi-line or single-line?** — Multi-line. The note editor overlay must support multi-line text input with line-by-line cursor navigation.
- **Notes List overlay?** — Yes. A browse overlay (similar to Bookmarks List) to view and navigate to all annotated verses. Keybinding TBD during implementation.

## References

- [Bookmarks implementation](../../src/config/bookmarks.rs) — pattern to follow for persistence
- [Overlay system](../../src/ui/browser.rs) — `OverlayKind` enum for modal overlays
- GitHub Issue: #2
