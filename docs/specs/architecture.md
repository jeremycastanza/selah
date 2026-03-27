# Technical Specification: Architecture Overview

_Created: 2026-03-26_
_Status: Draft_

## Overview

Selah is a single-binary, fully offline Rust TUI application for reading the Bible in the terminal. It unifies features from two source forks (`christ-cli` in Rust and `bible-tui` in JavaScript) into a clean, fresh Rust codebase.

## System Design

```
┌─────────────────────────────────────────┐
│               UI Layer                  │
│  Ratatui + crossterm (render + events)  │
├─────────────────────────────────────────┤
│             App / State Layer           │
│  AppMode enum, BrowserState, dispatch   │
├─────────────────────────────────────────┤
│              Data Layer                 │
│  rusqlite (bundled SQLite), serde/JSON  │
└─────────────────────────────────────────┘
```

## Module Map

Target `src/` layout:

```
src/
├── main.rs           — Entry point; CLI parsing via clap, TUI bootstrap
├── app.rs            — App struct, AppMode enum, main run loop, event dispatch
├── ui/
│   ├── mod.rs
│   ├── banner.rs     — Splash screen rendering + BannerState
│   ├── browser.rs    — 4-panel reader rendering + BrowserState
│   ├── search.rs     — Search overlay rendering + SearchState
│   ├── bookmarks.rs  — Bookmark list overlay rendering
│   └── theme.rs      — Theme tokens, ThemeName enum, built-in palettes
├── bible/
│   ├── mod.rs
│   ├── db.rs         — SQLite connection, query helpers (rusqlite)
│   ├── books.rs      — Static book metadata (name, chapters, testament)
│   ├── types.rs      — Verse, Chapter, SearchResult, BookmarkEntry types
│   └── random.rs     — Random verse selection logic
├── config/
│   ├── mod.rs
│   ├── session.rs    — Session state persistence (position, theme, translation)
│   └── bookmarks.rs  — Bookmark file persistence
└── cli.rs            — clap CLI struct and subcommand definitions
```

## Key Crate Choices

| Crate | Version | Purpose | Rationale |
|---|---|---|---|
| `ratatui` | 0.29 | TUI rendering | Largest Rust TUI ecosystem; already in `christ` fork; crossterm backend gives mouse support on macOS/Linux |
| `crossterm` | 0.28 | Terminal backend + mouse events | Pairs with ratatui; consistent macOS/Linux support; no unsafe |
| `rusqlite` | 0.32 | SQLite access | `bundled` feature compiles SQLite from source — zero system dependency |
| `serde` + `serde_json` | 1 | Session/bookmark serialization | Standard Rust serde ecosystem |
| `clap` | 4 | CLI argument parsing | Derive-based; already used in `christ` fork |
| `directories` | 6 | XDG/platform-aware config paths | Used in `christ` fork; handles macOS/Linux correctly |
| `rand` | 0.9 | Random verse selection | `rand::thread_rng` is sufficient; no crypto-grade randomness needed |
| `unicode-width` | 0.2 | Correct CJK/Unicode column widths | Already in `christ` fork; needed for multi-language translations |

### Crates Removed vs. `christ-cli`

| Crate | Reason Dropped |
|---|---|
| `tokio` / `reqwest` | Selah is fully offline; no async HTTP needed |
| `tachyonfx` | Animated effects crate; deferred to v2 |
| `confy` | Replaced by manual serde_json to directories path |
| `once_cell` | Replaced by Rust 1.70+ `std::sync::OnceLock` |

## App State Machine

```
┌────────────┐   banner done / keypress   ┌──────────────┐
│   Banner   │ ─────────────────────────► │   Browser    │
└────────────┘                            └──────┬───────┘
                                                 │
                            ┌────────────────────┼────────────────────┐
                            ▼                    ▼                    ▼
                     SearchOverlay      BookmarkOverlay    TranslationPicker
                     (Esc to close)     (Esc to close)     (Esc/v to close)
```

`AppMode` enum:

```rust
enum AppMode {
    Banner(BannerState),
    Browser(BrowserState),
}
```

Overlays are modeled as `Option<OverlayKind>` within `BrowserState` rather than top-level `AppMode` variants, keeping the event dispatch logic flat.

## References

- [Data Layer spec](data-layer.md)
- [UI Layout spec](ui-layout.md)
- [Build & Distribution spec](build-distribution.md)
- [`docs/decisions.md`](../decisions.md) — ADRs for language, framework, data storage choices
