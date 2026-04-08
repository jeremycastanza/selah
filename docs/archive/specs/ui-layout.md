# Technical Specification: UI Layout and Navigation

_Created: 2026-03-26_
_Status: Draft_

## Overview

Selah's UI is built with Ratatui 0.29 and the crossterm backend. The primary view is a 4-panel reader layout with an always-visible status bar. Overlays (search, bookmarks, translation picker) render on top of the browser using `Clear` + `Block`.

## TUI Framework

| Component | Choice | Rationale |
|---|---|---|
| Rendering | Ratatui 0.29 | Largest Rust TUI ecosystem; direct port from `christ-cli`; constraint-based layout suits 4-panel design |
| Backend | crossterm 0.28 | Mouse event support; macOS/Linux compatible; paired with ratatui |

The event loop runs at:
- **Banner mode:** 16ms tick (~60fps for smooth animation)
- **Browser mode:** 50ms tick (~20fps, sufficient for keyboard/mouse responsiveness)

## 4-Panel Layout

```
┌────────────────┬──────────┬──────────────────────────────────────┐
│  [*] Books     │ Chapters │                                      │
│   25% wide     │  17%     │         Scripture                    │
│                ├──────────┤           58% wide                   │
│                │  Verses  │                                      │
│                │  17%     │                                      │
├────────────────┴──────────┴──────────────────────────────────────┤
│  Status bar (1 row): keybindings + translation + theme name      │
└──────────────────────────────────────────────────────────────────┘
```

Column constraints:
```rust
[Constraint::Percentage(25), Constraint::Percentage(17), Constraint::Percentage(58)]
```

Middle column splits vertically 50/50 between Chapters and Verses.

Status bar: `Constraint::Length(1)` at the bottom of the outer vertical layout.

### Panel Descriptions

- **Books panel** (25%): scrollable `List` of all 66 Bible books. Active panel has highlighted border and `[*]` label; inactive panels show `[ ]` label.
- **Chapters panel** (top half of middle column, 17%): list of chapter numbers for the selected book.
- **Verses panel** (bottom half of middle column, 17%): list of verse numbers for the selected chapter.
- **Scripture panel** (58%): `Paragraph` with word-wrap for the current chapter's text. Scrollable. Verse numbers are displayed inline (e.g., `1. In the beginning...`).
- **Status bar** (1 row): always visible. Shows: active keybinding hints | current translation | current theme name.

### Panel Focus Indicator

```
Active panel:  border color = theme.border_active, label = " [*] Books "
Inactive panel: border color = theme.border,        label = " [ ] Books "
```

## Navigation Model

### Keyboard

| Key | Action |
|---|---|
| `h` / `←` | Move focus to left panel |
| `l` / `→` | Move focus to right panel; triggers chapter load if moving to Scripture |
| `j` / `↓` | Move selection down in active list panel |
| `k` / `↑` | Move selection up in active list panel |
| `Enter` | Confirm selection; equivalent to `l` |
| `q` (double) | Quit — first press shows "press q again to quit" in status bar; second press exits |
| `/` | Open search overlay |
| `t` | Cycle to next theme |
| `v` | Open translation picker overlay |
| `b` | Bookmark current verse (or open bookmarks if already at Scripture panel) |
| `B` | Open bookmarks overlay |
| `r` | Jump to a random verse |
| `Esc` | Close any open overlay |

### Mouse

Mouse support is enabled via `crossterm::event::EnableMouseCapture` during terminal init.

**Hit testing:** Panel `Rect` values computed during each `draw()` call are stored in `BrowserState` fields (`books_rect`, `chapters_rect`, `verses_rect`, `scripture_rect`). During mouse event handling, the event's `(column, row)` is checked against these stored rects.

| Mouse Event | Action |
|---|---|
| `MouseButton::Left` click in Books rect | Focus Books panel, select item at row |
| `MouseButton::Left` click in Chapters rect | Focus Chapters panel, select item |
| `MouseButton::Left` click in Verses rect | Focus Verses panel, select item, load chapter |
| `ScrollUp` in Scripture rect | Scroll scripture up |
| `ScrollDown` in Scripture rect | Scroll scripture down |
| `MouseButton::Left` click in overlay list | Select overlay item |

### Overlay Rendering

Overlays use `ratatui::widgets::Clear` to blank the region before rendering the overlay widget. This avoids artifacts from the underlying browser layout.

```
AppMode::Browser (always rendered)
  │
  ├── OverlayKind::Search       — rendered over lower portion of browser
  ├── OverlayKind::Bookmarks    — centered modal
  └── OverlayKind::Translation  — right-aligned modal
```

`BrowserState` holds:

```rust
pub overlay: Option<OverlayKind>,
```

```rust
pub enum OverlayKind {
    Search(SearchState),
    Bookmarks(BookmarkListState),
    Translation(TranslationPickerState),
}
```

## Theme System

Themes use 11 semantic color tokens:

```rust
pub struct Theme {
    pub bg:             Color,  // App background
    pub surface:        Color,  // Panel/widget background
    pub border:         Color,  // Inactive border
    pub border_active:  Color,  // Active panel border
    pub text:           Color,  // Primary text
    pub text_dim:       Color,  // Secondary text
    pub text_muted:     Color,  // Hint/disabled text
    pub accent:         Color,  // High-contrast accent (titles, selected)
    pub accent_soft:    Color,  // Medium-contrast accent
    pub highlight_bg:   Color,  // Selected item background
    pub search_match:   Color,  // Search match highlight color
}
```

Five built-in themes (ported directly from `christ-cli/src/ui/theme.rs`):

| Name | Style |
|---|---|
| Slate | Cool blue-gray dark (default) |
| Midnight | Pure black, neutral grays (shadcn/Vercel style) |
| Parchment | Warm cream/sepia, comfortable for long reading |
| Gospel | Clean bright white, crisp and minimal |
| Terminal | Transparent — uses the terminal's own background |

The active theme is resolved at render time: `let theme = get_theme(app.theme_name)`. No widget re-init is needed when switching themes.

## References

- [Architecture spec](architecture.md)
- `christ-cli` source: `src/ui/browser.rs`, `src/ui/theme.rs`, `src/ui/banner.rs`
- `bible-tui` source: `utils/ui.js` (layout proportions reference)
