# Technical Specification: Confirm to Close

_Created: 2026-04-15_
_Status: Draft_
_Author: AI-assisted_
_GitHub Issue: #5_

## Overview

Replace the current double-`q` quit mechanism with a proper confirmation dialog overlay, preventing accidental closure of the TUI.

## Requirements

### Functional Requirements

1. **FR-1**: Pressing `q` opens a confirmation overlay instead of setting a pending quit flag
2. **FR-2**: The overlay displays "Are you sure you want to quit?" with `y`/`n` options
3. **FR-3**: `y` or `Enter` confirms quit; `n` or `Esc` dismisses the overlay
4. **FR-4**: The confirmation overlay follows the existing overlay visual style

### Non-Functional Requirements

1. **NFR-1**: No additional latency — overlay renders in the same frame
2. **NFR-2**: No `unsafe` Rust

## Technical Design

### Architecture

This replaces the `quit_pending: bool` flag in `App` with a new `OverlayKind::QuitConfirm` variant. The overlay system already intercepts all key events when active, so no special event routing is needed.

### Current Behavior

`app.rs:275-282`: Pressing `q` sets `quit_pending = true`. Pressing `q` again (with `quit_pending` already true) sets `should_quit = true`. The status bar shows "press q again to quit" when pending.

### New Behavior

Pressing `q` opens `OverlayKind::QuitConfirm`. Inside the overlay:
- `y` / `Enter` -> `should_quit = true`
- `n` / `Esc` / `q` -> close overlay, resume browsing

### Component Design

| Component | File | Responsibility |
|-----------|------|----------------|
| `OverlayKind::QuitConfirm` | `src/ui/browser.rs` | New overlay variant (no state needed) |
| `render_quit_confirm` | `src/ui/quit_confirm.rs` | Render confirmation dialog |
| Key dispatch | `src/app.rs` | Handle `q` -> open overlay; handle `y`/`n` inside overlay |

### Overlay Design

A small centered modal (roughly 40x5 characters):

```
+-- Quit --------------------------------+
|                                        |
|  Are you sure you want to quit?        |
|          [Y]es   [N]o                  |
|                                        |
+----------------------------------------+
```

Uses `theme.surface` background, `theme.border_active` border, `theme.text` for body, `theme.accent` for key hints.

### Removed State

The `quit_pending: bool` field on `App` and the "press q again to quit" status bar message are removed.

## Dependencies

No new dependencies required.

## Alternatives Considered

### Option A: Keep double-`q` but add visual feedback

Improve the current pattern with a more prominent flash or countdown.

- **Pros**: No overlay needed; simpler
- **Cons**: Still easy to accidentally quit by pressing `q` twice quickly; doesn't match user expectation from issue

### Option B: Confirmation overlay

- **Pros**: Explicit intent; matches the user story; uses existing overlay system
- **Cons**: Slightly heavier UX for intentional quits

### Decision

Option B — confirmation overlay. The user explicitly requested "confirm to close" behavior. The overlay is lightweight and consistent with the existing modal system.

## Security Considerations

None — this is a UI-only change with no data or network implications.

## Testing Strategy

- Unit tests: `q` key opens `QuitConfirm` overlay; `y` sets `should_quit`; `n`/`Esc` closes overlay
- Integration tests: Pressing `q` then `n` returns to browser; pressing `q` then `y` exits
- Manual verification: Overlay renders correctly across all 5 themes

## Resolved Questions

- **`Ctrl+C` behavior?** — Remains an immediate exit. The confirmation dialog only applies to the `q` key.

## References

- [Current quit handling](../../src/app.rs:275) — `quit_pending` logic
- [Overlay system](../../src/ui/browser.rs:49) — `OverlayKind` enum
- GitHub Issue: #5
