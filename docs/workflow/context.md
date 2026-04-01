# Project Context

_Last updated: 2026-04-01_
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
│   │   ├── banner.rs     # BannerState — splash screen (placeholder)
│   │   ├── browser.rs    # BrowserState, Panel, OverlayKind, render_browser, hit_test
│   │   ├── search.rs     # SearchState, render_search
│   │   ├── bookmarks.rs  # stub (Phase 7)
│   │   └── theme.rs      # Theme, ThemeName (5 themes), get_theme, interpolate_color
│   ├── bible/            # Bible data models and DB access
│   │   ├── mod.rs
│   │   ├── db.rs         # open_db, get_chapter, get_verse, search, get_random_verse
│   │   ├── books.rs      # BOOKS array (66 books), book_name()
│   │   ├── types.rs      # Verse, Chapter, SearchResult
│   │   └── random.rs     # thin wrapper over db::get_random_verse
│   └── config/           # Session and bookmark persistence
│       ├── mod.rs
│       ├── session.rs    # SessionState, load(), save()
│       └── bookmarks.rs  # stub (Phase 7)
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

All Bible data is bundled with the binary. `data/kjv.sqlite` is embedded via `include_bytes!` at compile time and written to a temp file on startup. No network requests at runtime.

### FTS5 Search

The full-text search index is built at runtime from the verses table — it is not stored on disk. Activated with `/` in the TUI; results navigate directly to the matching verse.

### Session Persistence

`SessionState` (book index, chapter, verse) is serialized to JSON and written to the platform data directory via the `directories` crate. Loaded on startup, saved on clean exit.

## Tech Stack

| Layer | Technology |
|-------|------------|
| Language | Rust |
| TUI | ratatui + crossterm |
| Data | SQLite via rusqlite (`bundled`), sourced from scrollmapper/bible_databases |
| CLI | clap |
| Session storage | serde_json + directories crate |
| Distribution | Private Homebrew tap (planned Phase 9) |

## Critical Files

| File | Why it matters |
|------|---------------|
| `src/app.rs` | Owns all runtime state; central dispatch for all events |
| `src/ui/browser.rs` | Primary reading UI; hit_test drives mouse routing |
| `src/bible/db.rs` | All Bible data access; FTS5 search lives here |
| `src/config/session.rs` | Session load/save; defines persistence contract |
| `src/ui/theme.rs` | All color tokens; changing themes touches this |
| `data/kjv.sqlite` | The Bible data; embedded at compile time |

## Data Shapes

```rust
// bible/types.rs
pub struct Verse {
    pub book: u8,
    pub chapter: u8,
    pub verse: u8,
    pub text: String,
}

pub struct Chapter {
    pub verses: Vec<Verse>,
}

pub struct SearchResult {
    pub book: u8,
    pub chapter: u8,
    pub verse: u8,
    pub text: String,
    pub snippet: String,
}

// config/session.rs
pub struct SessionState {
    pub book: usize,
    pub chapter: usize,
    pub verse: usize,
}
```

## Current Work

See `docs/workflow/scope.md` for the current iteration.

## Constraints

- Must compile and run on macOS and Linux
- No `unsafe` Rust
- Must work fully offline — no runtime network dependencies
- Keep this file under 200 lines
