# Technical Specification: Verse Highlights

_Created: 2026-04-15_
_Status: Draft_
_Author: AI-assisted_
_GitHub Issue: #3_

## Overview

Add the ability for users to highlight verses with a color, making important passages visually distinct in the Scripture panel. Highlights are persisted locally and rendered inline when viewing chapters.

## Requirements

### Functional Requirements

1. **FR-1**: User can highlight the currently selected verse with a color
2. **FR-2**: User can cycle through available highlight colors
3. **FR-3**: User can remove a highlight from a verse
4. **FR-4**: Highlighted verses are visually distinct in the Scripture panel (background or text color)
5. **FR-5**: Highlights persist across sessions
6. **FR-6**: Highlights work across all themes (colors adapt or remain readable)

### Non-Functional Requirements

1. **NFR-1**: Fully offline — no network dependencies
2. **NFR-2**: Rendering highlighted verses must not introduce visible lag
3. **NFR-3**: No `unsafe` Rust

## Technical Design

### Architecture

Highlights follow the same persistence pattern as bookmarks and notes — a `config/highlights.rs` module with a JSON file. The Scripture panel rendering in `ui/browser.rs` checks for highlights when building verse `Span`s.

### Data Model

```rust
// config/highlights.rs
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum HighlightColor {
    Yellow,
    Green,
    Blue,
    Pink,
    Orange,
}

pub struct HighlightEntry {
    pub book: String,
    pub chapter: u32,
    pub verse: u32,
    pub color: HighlightColor,
    pub created_at: u64,
}
```

Stored as `highlights.json` in the platform data directory.

### Key Identity

A highlight is uniquely identified by `(book, chapter, verse)`. One highlight per verse — toggling cycles the color or removes it.

### Color Mapping

Each `HighlightColor` maps to a `ratatui::style::Color` that is readable against all 5 theme backgrounds. The mapping lives in `ui/theme.rs` as a method on `Theme`:

```rust
impl Theme {
    pub fn highlight_color(&self, color: HighlightColor) -> Color {
        // Returns a muted background color that works with self.text
    }
}
```

### Component Design

| Component | File | Responsibility |
|-----------|------|----------------|
| `HighlightEntry` / `HighlightColor` | `src/config/highlights.rs` | Data model + load/save/toggle |
| Scripture rendering | `src/ui/browser.rs` | Apply highlight background to verse spans |
| Key dispatch | `src/app.rs` | `H` to toggle highlight on selected verse |

### Keybindings

| Key | Context | Action |
|-----|---------|--------|
| `H` | Browser, verse selected | Cycle highlight color (None -> Yellow -> Green -> ... -> None) |

### Scripture Panel Integration

In `render_browser`, when building `scripture_lines`, look up each verse in the highlights list. If highlighted, set the `Span` background to the theme-resolved highlight color.

## Dependencies

No new dependencies required. Uses existing `serde`, `serde_json`, `directories`.

## Alternatives Considered

### Option A: Store highlights in SQLite

Add a `highlights` table to the embedded database.

- **Pros**: Efficient lookups by verse
- **Cons**: The DB is read-only (embedded via `include_bytes!`); would need a separate writable DB, adding complexity

### Option B: JSON file (same pattern as bookmarks)

- **Pros**: Consistent with existing persistence; simple implementation
- **Cons**: O(n) lookup per verse when rendering a chapter

### Decision

Option B — JSON file. For the expected scale (hundreds of highlights, not millions), O(n) lookup is negligible. A `HashMap<(String, u32, u32), HighlightColor>` built at load time makes rendering O(1) per verse.

## Security Considerations

- Local-only storage, no network
- No user text input (just toggling a color enum)

## Testing Strategy

- Unit tests: `config/highlights.rs` — add, toggle color, remove, dedup, serde round-trip
- Unit tests: `Theme::highlight_color` — all colors readable against all 5 themes
- Integration tests: Highlight a verse, reload, verify persistence and rendering
- Manual verification: Highlighted verses visually distinct across all themes

## Resolved Questions

- **Highlights in Verses panel?** — No. Highlights render in the Scripture panel only.
- **Highlights List overlay?** — Yes. A browse overlay (similar to Bookmarks List) to view and navigate to all highlighted verses. Keybinding TBD during implementation.
- **Highlight visibility toggle?** — Yes. Users should be able to toggle highlight rendering on/off globally without deleting the highlight data.

## References

- [Bookmarks implementation](../../src/config/bookmarks.rs) — persistence pattern
- [Theme system](../../src/ui/theme.rs) — color token pattern
- [Scripture rendering](../../src/ui/browser.rs:451) — verse span construction
- GitHub Issue: #3
