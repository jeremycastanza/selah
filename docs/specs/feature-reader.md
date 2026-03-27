# Technical Specification: F1 — Clean TUI Interface

_Created: 2026-03-26_
_Status: Draft_

## Overview

The core interactive reader: a 4-panel TUI layout with Books, Chapters, Verses, and Scripture panels, plus a status bar. This is the primary view users spend all their time in.

## Requirements

### Functional

- **FR-1**: Four panels rendered side-by-side: Books (25%), Chapters+Verses stacked (17%), Scripture (58%)
- **FR-2**: One panel is always "active" (focused), indicated by highlighted border and `[*]` label
- **FR-3**: Navigation moves focus left/right between panels; up/down moves selection within a panel
- **FR-4**: Selecting a chapter causes the Scripture panel to reload with that chapter's text
- **FR-5**: Scripture panel is word-wrapped and vertically scrollable
- **FR-6**: Status bar (1 row, always visible) shows keybinding hints, current translation, current theme
- **FR-7**: Session state (book, chapter, scroll position, active panel) is restored on next launch

### Non-Functional

- **NFR-1**: Chapter load from SQLite must feel instant (< 50ms)
- **NFR-2**: Layout adapts to terminal width — panels maintain percentage-based widths

## User Interaction Flow

1. App launches (after banner or with `--no-banner`)
2. Books panel is focused; book list is populated from static `BOOKS` data
3. User presses `j/k` to move through books, `l/Enter` to focus Chapters panel
4. Chapters panel populates based on selected book; user selects chapter
5. `l/Enter` loads selected chapter's verse text into Scripture panel and focuses Scripture
6. In Scripture panel, `j/k` scroll the text; `h/←` returns focus to Verses/Chapters
7. On quit (`q` double), session state is saved

## Technical Design

### BrowserState

All reader view state lives in `BrowserState` (in `ui/browser.rs`):

```rust
pub struct BrowserState {
    pub active_panel: Panel,
    pub book_list: ListState,
    pub chapter_list: ListState,
    pub verse_list: ListState,
    pub scripture_scroll: u16,
    pub selected_book_idx: usize,
    pub selected_chapter: u32,
    pub selected_verse: u32,
    pub current_chapter: Option<Chapter>,
    pub loading: bool,
    pub translation: String,
    pub overlay: Option<OverlayKind>,
    // For mouse hit-testing (updated each draw() call):
    pub books_rect: Rect,
    pub chapters_rect: Rect,
    pub verses_rect: Rect,
    pub scripture_rect: Rect,
    // Brief status bar messages:
    pub status_flash: Option<(String, Instant)>,
}
```

### Panel Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Panel {
    Books,
    Chapters,
    Verses,
    Scripture,
}
```

### Rendering

`ui/browser.rs` exposes:

```rust
pub fn render_browser(frame: &mut Frame, area: Rect, state: &mut BrowserState, quit_pending: bool, theme: &Theme)
```

Layout computed per frame:
1. Outer vertical split: `[Constraint::Min(0), Constraint::Length(1)]` — browser area + status bar
2. Browser area horizontal split: `[Percentage(25), Percentage(17), Percentage(58)]`
3. Middle column vertical split: `[Percentage(50), Percentage(50)]` — Chapters + Verses

Panel rects are stored back into `state` for mouse hit-testing after layout computation.

### Session Restore

On startup, `config/session.rs::load()` returns the last `SessionState`. `BrowserState::restore(saved)` applies it — restoring `selected_book_idx`, `selected_chapter`, `active_panel`, `scripture_scroll`.

## Dependencies

- `bible/db.rs` — chapter queries
- `bible/books.rs` — static book list
- `ui/theme.rs` — theme tokens
- `config/session.rs` — session save/load
- All overlay features (F4, F7, F8) render on top of this layout

## Testing Strategy

- **Manual**: Navigate from Genesis 1 to Revelation 22 using only keyboard
- **Manual**: Confirm status bar updates on panel switch, translation change, theme change
- **Manual**: Quit and relaunch — confirm position is restored
- **Manual**: Resize terminal — confirm layout reflows correctly
- **Automated**: Unit test `BrowserState` navigation methods (`next_panel`, `prev_panel`, `move_up`, `move_down`)

## References

- `christ-cli` source: `src/ui/browser.rs`, `src/app.rs`
- `bible-tui` source: `utils/ui.js` (4-panel layout reference)
- [UI Layout spec](ui-layout.md)
