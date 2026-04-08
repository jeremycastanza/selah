# Technical Specification: F6 — Theme Toggle

_Created: 2026-03-26_
_Status: Draft_

## Overview

Cycle through 5 built-in themes with a single key press. The active theme is shown in the status bar and persisted across sessions.

## Requirements

### Functional

- **FR-1**: Press `t` in browser mode to cycle to the next theme
- **FR-2**: Theme change takes effect immediately on the next render frame (no restart required)
- **FR-3**: The current theme name is shown in the status bar at all times
- **FR-4**: The selected theme persists across restarts (stored in session state)
- **FR-5**: Five built-in themes: Slate, Midnight, Parchment, Gospel, Terminal

### Non-Functional

- **NFR-1**: Theme switching must not cause any flicker or re-init cost

## User Interaction Flow

1. User is in the browser
2. User presses `t` — theme cycles to the next one (Slate → Midnight → Parchment → Gospel → Terminal → Slate)
3. All panels, borders, text, and status bar instantly re-render with new colors
4. Status bar shows the new theme name (e.g., `Theme: Midnight`)
5. On next launch, the same theme is restored

## Technical Design

### Theme Tokens

All 11 semantic color tokens are defined in `ui/theme.rs` (direct port from `christ-cli`):

```rust
pub struct Theme {
    pub bg: Color,
    pub surface: Color,
    pub border: Color,
    pub border_active: Color,
    pub text: Color,
    pub text_dim: Color,
    pub text_muted: Color,
    pub accent: Color,
    pub accent_soft: Color,
    pub highlight_bg: Color,
    pub search_match: Color,
}
```

### ThemeName Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum ThemeName {
    #[default]
    Slate,
    Midnight,
    Parchment,
    Gospel,
    Terminal,
}

impl ThemeName {
    pub fn next(self) -> Self { /* cycle */ }
    pub fn label(self) -> &'static str { /* "Slate", "Midnight", etc. */ }
}
```

### App Integration

`App` holds the active theme name:

```rust
pub struct App {
    // ...
    pub theme_name: ThemeName,
}
```

In `handle_key`:
```rust
KeyCode::Char('t') => {
    self.theme_name = self.theme_name.next();
}
```

In `draw()`:
```rust
let theme = get_theme(self.theme_name);
render_browser(frame, area, state, &theme);
```

Theme is resolved at render time — no state mutation in the render path, no re-init.

### Persistence

`ThemeName` is included in `SessionState`:

```rust
pub struct SessionState {
    // ...
    pub theme: ThemeName,
}
```

Loaded on startup, saved on quit.

### Built-in Themes

| Name | Style | Background |
|---|---|---|
| Slate | Cool blue-gray dark (default) | `#0f172a` |
| Midnight | Pure black, shadcn/Vercel style | `#000000` |
| Parchment | Warm cream/sepia | `#f5f0e1` |
| Gospel | Clean bright white | `#ffffff` |
| Terminal | Transparent (uses terminal background) | `Color::Reset` |

All palette values are ported verbatim from `christ-cli/src/ui/theme.rs`.

## Dependencies

- F1 (all browser rendering functions receive `&Theme`)
- F5 (banner also receives `&Theme` for color interpolation)
- `config/session.rs` (`ThemeName` stored in `SessionState`)

## Testing Strategy

- **Manual**: Press `t` repeatedly — verify all 5 themes cycle through and display correctly
- **Manual**: Switch to Parchment, quit, relaunch — confirm Parchment is active
- **Manual**: Verify Terminal theme uses terminal background (no hardcoded dark bg)
- **Automated**: Unit test `ThemeName::next()` cycles correctly including wrap-around (Terminal → Slate)
- **Automated**: Unit test serde round-trip for `ThemeName` (serialize and deserialize all 5 variants)

## References

- `christ-cli` source: `src/ui/theme.rs` (direct reference implementation — all palette values)
- `christ-cli` source: `src/app.rs` (theme cycling via `KeyCode::Char('t')`)
