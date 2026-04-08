# Project Context

_Last updated: 2026-04-08_
_Target: ~200 lines_

## What This Is

Selah is a terminal user interface (TUI) for the Holy Bible, built in Rust. It targets Christian developers who live in the terminal and want to read, navigate, and study scripture without leaving their workflow. The name "Selah" is a Hebrew word found in Psalms, thought to mean a pause or rest — fitting for a tool meant for quiet reflection.

## Directory Map

```
selah/
├── src/
│   ├── main.rs           # Entry point; CLI dispatch (clap)
│   ├── app.rs            # Application state machine and event loop
│   ├── cli.rs            # clap CLI (Commands::Random)
│   ├── ui/               # TUI rendering and layout
│   │   ├── mod.rs
│   │   ├── banner.rs     # BannerState — animated splash screen (Ichthys art + SELAH title)
│   │   ├── browser.rs    # BrowserState, Panel, OverlayKind, render_browser, hit_test
│   │   ├── search.rs     # SearchState, render_search
│   │   ├── bookmarks.rs  # BookmarkListState, render_bookmarks (modal overlay)
│   │   ├── translation.rs# TranslationPickerState, render_translation_picker (modal overlay)
│   │   └── theme.rs      # Theme, ThemeName (5 themes), get_theme, interpolate_color
│   ├── bible/            # Bible data models and DB access
│   │   ├── mod.rs        # TranslationInfo, TRANSLATIONS static array
│   │   ├── db.rs         # open_db, get_chapter, get_verse, search, get_random_verse
│   │   ├── books.rs      # BOOKS array (66 books), book_name()
│   │   ├── types.rs      # Verse, Chapter, SearchResult
│   │   └── random.rs     # thin wrapper over db::get_random_verse
│   └── config/           # Session and bookmark persistence
│       ├── mod.rs
│       ├── session.rs    # SessionState, load(), save()
│       └── bookmarks.rs  # BookmarkEntry, load(), save(), add(), remove(), now_unix()
├── data/
│   └── kjv.sqlite        # KJV Bible database (embedded via include_bytes!)
├── tests/                # Integration tests
├── docs/                 # Documentation system
│   ├── workflow/         # AI collaboration files
│   ├── rules/            # AI constraints
│   ├── plans/            # Implementation plans
│   ├── specs/            # Feature specifications
│   └── technical/        # Domain reference docs
├── Cargo.toml            # Package manifest
└── Cargo.lock            # Dependency lock file
```

## Key Patterns

### 4-Panel Layout

The UI is modeled after `bible-tui`'s layout: **Books (25%) | Chapters+Verses stacked (17%) | Scripture (58%)**. Navigation flows left-to-right: Book → Chapter → Verse → Scripture. The active panel is indicated by a highlighted border and `[*]` label. An always-visible status bar at the bottom shows keybinding hints.

### Theme System

Themes use semantic tokens (`bg`, `surface`, `border`, `border_active`, `text`, `text_dim`, `accent`, `highlight_bg`, `search_match`) rather than raw per-element colors. Inherited from `christ-cli`. Five built-in themes: Slate, Midnight, Parchment, Gospel, Terminal. Active theme toggled with `t`.

### Fully Offline

All Bible data is bundled with the binary. `data/kjv.sqlite` is embedded via `include_bytes!` at compile time and loaded into an in-memory SQLite connection on startup. No network requests at runtime.

### FTS5 Search

The full-text search index is built at runtime from the verses table — it is not stored on disk. Activated with `/` in the TUI; results navigate directly to the matching verse.

### Overlay System

Three modal overlays share a single `OverlayKind` enum in `BrowserState`. Only one can be active at a time. Key events are intercepted entirely by the active overlay; mouse is ignored while an overlay is open.

- `OverlayKind::Search(SearchState)` — `/` key, FTS5 search
- `OverlayKind::Bookmarks(BookmarkListState)` — `B` key, saved verse list
- `OverlayKind::Translation(TranslationPickerState)` — `v` key, Bible version picker

### Splash Screen Animation

`BannerState` drives a tick-based phase machine at 16ms/tick (~60fps):

- Phase 0 (ticks 0–50): Ichthys ASCII art fades in (2-color: blue body, yellow detail)
- Phase 1 (ticks 51–95): SELAH block title fades in to `theme.accent`
- Phase 2 (ticks 96–140): Tagline typewriter effect
- Phase 3 (ticks 141–300): Settle; `done = true` at tick 301

Any keypress skips directly to browser. `?` in browser replays the splash.

### Session Persistence

`SessionState` (book index, chapter, verse, scroll, theme, translation) is serialized to JSON and written to the platform data directory via the `directories` crate. Loaded on startup, saved on clean exit.

## Tech Stack

| Layer | Technology |
|-------|------------|
| Language | Rust 2024 edition |
| TUI | ratatui 0.29 + crossterm |
| Data | SQLite via rusqlite (`bundled`), sourced from scrollmapper/bible_databases |
| CLI | clap |
| Session storage | serde_json + directories crate |
| Distribution | cargo-dist + private Homebrew tap (`jeremycastanza/homebrew-selah-tap`) |

## Critical Files

| File | Why it matters |
|------|---------------|
| `src/app.rs` | Owns all runtime state; central dispatch for all events and overlays |
| `src/ui/browser.rs` | Primary reading UI; hit_test drives mouse routing; OverlayKind lives here |
| `src/bible/db.rs` | All Bible data access; FTS5 search lives here |
| `src/config/session.rs` | Session load/save; defines persistence contract |
| `src/config/bookmarks.rs` | Bookmark persistence; dedup logic |
| `src/ui/theme.rs` | All color tokens; changing themes touches this |
| `data/kjv.sqlite` | The Bible data; embedded at compile time |

## Data Shapes

```rust
// bible/types.rs
pub struct Verse {
    pub book: String,
    pub book_num: u32,
    pub chapter: u32,
    pub verse: u32,
    pub text: String,
    pub translation: String,
}

pub struct Chapter {
    pub book: String,
    pub chapter: u32,
    pub verses: Vec<Verse>,
}

pub struct SearchResult {
    pub book: String,
    pub book_num: u32,
    pub chapter: u32,
    pub verse: u32,
    pub text: String,
}

// config/session.rs
pub struct SessionState {
    pub book_index: usize,
    pub chapter: u32,
    pub scroll_position: u16,
    pub active_panel: u8,
    pub theme: ThemeName,
    pub translation: String,
}

// config/bookmarks.rs
pub struct BookmarkEntry {
    pub book: String,
    pub chapter: u32,
    pub verse: u32,
    pub snippet: Option<String>,
    pub note: Option<String>,
    pub created_at: u64,
}

// bible/mod.rs
pub struct TranslationInfo {
    pub code: &'static str,
    pub name: &'static str,
    pub lang: &'static str,
    pub offline: bool,
}
```

## Current Work

See `docs/workflow/scope.md` for the current iteration.

## Constraints

- Must compile and run on macOS and Linux
- No `unsafe` Rust
- Must work fully offline — no runtime network dependencies
- Keep this file under 200 lines
