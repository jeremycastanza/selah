# Technical Specification: F3 — Mouse Interaction

_Created: 2026-03-26_
_Status: Draft_

## Overview

Mouse clicks focus panels and select items. Scroll wheel scrolls the Scripture panel. This supplements keyboard navigation without replacing it.

## Requirements

### Functional

- **FR-1**: Left click on a Books panel row selects and focuses that book
- **FR-2**: Left click on a Chapters panel row selects and focuses that chapter
- **FR-3**: Left click on a Verses panel row selects and focuses that verse; triggers chapter load
- **FR-4**: Scroll up/down in the Scripture panel scrolls the text
- **FR-5**: Mouse clicks work in any overlay that contains a list (bookmarks, translation picker)

### Non-Functional

- **NFR-1**: Mouse input must not interfere with keyboard navigation
- **NFR-2**: Mouse capture is enabled/disabled cleanly on start/exit (no terminal state leakage)

## User Interaction Flow

1. User clicks a book name in the Books panel → Books panel becomes active, that book is selected
2. User clicks a chapter number → Chapters panel becomes active, chapter selected
3. User clicks a verse → Verses panel active, verse selected, chapter loads, Scripture updates
4. User scrolls in the Scripture panel → text scrolls up or down
5. User clicks a bookmark in the Bookmark overlay → navigates to that verse

## Technical Design

### Enabling Mouse Support

In `main.rs` terminal setup:

```rust
crossterm::execute!(stdout, crossterm::event::EnableMouseCapture)?;
```

On `ratatui::restore()`:
```rust
crossterm::execute!(stdout, crossterm::event::DisableMouseCapture)?;
```

### Event Dispatch

In the main event loop, mouse events are handled alongside keyboard events:

```rust
match event::read()? {
    Event::Key(key) if key.kind == KeyEventKind::Press => handle_key(key.code),
    Event::Mouse(mouse) => handle_mouse(mouse),
    _ => {}
}
```

### Hit Testing

Panel rects are stored in `BrowserState` and updated on each `draw()` call:

```rust
pub struct BrowserState {
    // ...
    pub books_rect: Rect,
    pub chapters_rect: Rect,
    pub verses_rect: Rect,
    pub scripture_rect: Rect,
}
```

Utility function:

```rust
fn hit_test(col: u16, row: u16, rect: Rect) -> bool {
    col >= rect.x && col < rect.x + rect.width &&
    row >= rect.y && row < rect.y + rect.height
}
```

### Mouse Handler

```rust
fn handle_mouse(mouse: MouseEvent, state: &mut BrowserState) {
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            let (col, row) = (mouse.column, mouse.row);
            if hit_test(col, row, state.books_rect) {
                state.active_panel = Panel::Books;
                let item_row = row.saturating_sub(state.books_rect.y + 1); // offset for border
                state.book_list.select(Some(item_row as usize));
            } else if hit_test(col, row, state.chapters_rect) {
                state.active_panel = Panel::Chapters;
                let item_row = row.saturating_sub(state.chapters_rect.y + 1);
                state.chapter_list.select(Some(item_row as usize));
            } else if hit_test(col, row, state.verses_rect) {
                state.active_panel = Panel::Verses;
                let item_row = row.saturating_sub(state.verses_rect.y + 1);
                state.verse_list.select(Some(item_row as usize));
                // Trigger chapter load
            }
        }
        MouseEventKind::ScrollUp => {
            if hit_test(mouse.column, mouse.row, state.scripture_rect) {
                state.scripture_scroll = state.scripture_scroll.saturating_sub(1);
            }
        }
        MouseEventKind::ScrollDown => {
            if hit_test(mouse.column, mouse.row, state.scripture_rect) {
                state.scripture_scroll = state.scripture_scroll.saturating_add(1);
            }
        }
        _ => {}
    }
}
```

### Scroll Bounds

Scripture scroll is clamped to the total number of lines in the current chapter minus the visible height. This prevents scrolling past the end of content.

## Dependencies

- F1 (panel rects must be stored in `BrowserState` during `draw()`)
- `crossterm` mouse event types (`MouseEvent`, `MouseEventKind`, `MouseButton`)

## Testing Strategy

- **Manual**: Click each panel — verify focus indicator moves to clicked panel
- **Manual**: Click items in Books/Chapters/Verses — verify selection updates correctly
- **Manual**: Scroll in Scripture panel with mouse wheel — verify text scrolls
- **Manual**: Quit and verify terminal mouse mode is restored (no leftover mouse capture)
- **Automated**: Unit test `hit_test` with boundary coordinates (inside, outside, on edge)

## References

- `bible-tui` source: `utils/ui.js` (`mouse: true` on blessed lists — confirms mouse selection was a feature)
- `christ-cli` source: `src/app.rs` (does not implement mouse — this is new in Selah)
- crossterm docs: `MouseEvent`, `EnableMouseCapture`
