# Technical Specification: F5 — Splash Screen

_Created: 2026-03-26_
_Status: Draft_

## Overview

An animated splash screen is shown on launch unless `--no-banner` is passed. It displays supplied artwork (sourced from an external project), the "SELAH" title in block letters, and a tagline using a typewriter effect — then automatically transitions to the browser.

## Requirements

### Functional

- **FR-1**: Full-color ASCII artwork fades in from background color (~800ms). Art is supplied from an external project — not a cross, and rendered with per-character RGB colors.
- **FR-2**: "SELAH" block-letter title fades in after cross (~700ms)
- **FR-3**: Tagline types in character-by-character after title (~700ms)
- **FR-4**: Screen settles for ~500ms, then transitions automatically to the reader
- **FR-5**: Any keypress skips the banner immediately
- **FR-6**: `--no-banner` / `-n` CLI flag launches directly into the reader

### Non-Functional

- **NFR-1**: Banner tick rate is 16ms (~60fps) for smooth animation
- **NFR-2**: Total banner duration: ~3 seconds uninterrupted

## User Interaction Flow

1. User runs `selah` (no flags)
2. Terminal clears; banner renders centered on screen
3. Artwork fades in
4. "SELAH" title fades in
5. Tagline types in letter by letter
6. Brief pause, then browser loads automatically
7. Alternatively: user presses any key → banner skips, browser loads immediately

## Technical Design

### Component

`ui/banner.rs` — direct port and adaptation of `christ-cli/src/ui/banner.rs`.

The banner does not use `tachyonfx`. All animation is driven by manual tick counting and a linear `interpolate_color` function (already present in the fork).

### State

```rust
// ui/banner.rs
pub struct BannerState {
    pub phase: u8,
    pub tick: u32,
    pub done: bool,
}

impl BannerState {
    pub fn tick(&mut self) {
        self.tick += 1;
        match self.tick {
            0..=50  => self.phase = 0,   // Cross fades in (~800ms at 16ms/tick)
            51..=95 => self.phase = 1,   // Title fades in (~700ms)
            96..=140 => self.phase = 2,  // Tagline types in (~700ms)
            141..=175 => self.phase = 3, // Settle (~560ms)
            _ => self.done = true,
        }
    }
}
```

### Artwork

Splash artwork is full-color ASCII art supplied from an external project and stored as a static constant in `ui/banner.rs`. The artwork uses RGB colors (ratatui `Color::Rgb`) and is rendered centered on screen. The art asset and its color data are provided as-is — not generated programmatically.

The `TITLE_ART` block-letter constant for "SELAH" is kept separately:

```
 ███████╗███████╗██╗      █████╗ ██╗  ██╗
 ██╔════╝██╔════╝██║     ██╔══██╗██║  ██║
 ███████╗█████╗  ██║     ███████║███████║
 ╚════██║██╔══╝  ██║     ██╔══██║██╔══██║
 ███████║███████╗███████╗██║  ██║██║  ██║
 ╚══════╝╚══════╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝
```

### Tagline

`"The Word, always with you"` — rendered with typewriter effect (one character revealed per tick during phase 2).

### Color Interpolation

```rust
pub fn interpolate_color(from: Color, to: Color, t: f32) -> Color {
    // Linear interpolation between two Rgb colors
    // Falls back to `to` if either color is non-Rgb
}
```

### Rendering Function

```rust
pub fn render_banner(frame: &mut Frame, area: Rect, state: &BannerState, theme: &Theme)
```

Layout: vertical centering of `[artwork | gap(1) | title(6) | gap(1) | tagline(1)]` using `Layout::vertical` with `Flex::Center`. Artwork height is derived from the supplied asset.

### Integration with App

In `app.rs`:

```rust
AppMode::Banner(ref mut state) => {
    state.tick();
    if state.done {
        self.mode = AppMode::Browser(BrowserState::new());
    }
}
```

On any keypress during Banner mode: `state.done = true`.

## Dependencies

- `ui/theme.rs` — theme tokens used for color interpolation
- `app.rs` — `AppMode::Banner` variant and transition to `AppMode::Browser`

## Testing Strategy

- **Manual**: Run `selah` and observe all 4 phases complete in ~3 seconds
- **Manual**: Run `selah` and press a key immediately — browser should appear instantly
- **Manual**: Run `selah --no-banner` — browser should appear without any splash animation
- **Automated**: Unit test `BannerState::tick()` transitions at boundary tick values (50, 51, 95, 96, 140, 141, 175, 176)

## References

- `christ-cli` source: `src/ui/banner.rs` (direct reference implementation)
