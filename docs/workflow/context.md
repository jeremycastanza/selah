# Project Context

_Last updated: 2026-03-25_
_Target: ~200 lines_

## What This Is

Selah is a terminal user interface (TUI) for the Holy Bible, built in Rust. It targets Christian developers who live in the terminal and want to read, navigate, and study scripture without leaving their workflow. The name "Selah" is a Hebrew word found in Psalms, thought to mean a pause or rest — fitting for a tool meant for quiet reflection.

## Directory Map

```
selah/
├── src/
│   ├── main.rs           # Entry point
│   ├── app.rs            # Application state and event loop
│   ├── ui/               # TUI rendering and layout
│   ├── bible/            # Bible data models and parsing
│   └── config.rs         # Configuration
├── tests/                # Integration tests
├── docs/                 # Documentation system
│   ├── workflow/         # AI collaboration files
│   ├── rules/            # AI constraints
│   ├── plans/            # Implementation plans
│   ├── specs/            # Technical specifications
│   └── technical/        # Domain reference docs
├── Cargo.toml            # Package manifest
└── Cargo.lock            # Dependency lock file
```

## Key Patterns

### 4-Panel Layout

The UI is modeled after `bible-tui`'s layout: **Books (25%) | Chapters+Verses stacked (17%) | Scripture (58%)**. Navigation flows left-to-right: Book → Chapter → Verse → Scripture. The active panel is indicated by a highlighted border and `[*]` label. An always-visible status bar at the bottom shows keybinding hints.

### Theme System

Themes use semantic tokens (bg, surface, border, border_active, text, text_dim, accent, highlight_bg, search_match) rather than raw per-element colors. Inherited from `christ-cli`. Five built-in themes: Slate, Midnight, Parchment, Gospel, Terminal.

### Fully Offline

All Bible data is bundled with the binary. No network requests at runtime. Multiple translations are supported via an embedded SQLite database compiled into the binary using `rusqlite` with the `bundled` feature.

## Tech Stack

| Layer | Technology |
|-------|------------|
| Language | Rust |
| TUI | Ratatui + crossterm |
| Data | SQLite via rusqlite (`bundled`), sourced from scrollmapper/bible_databases |
| Distribution | Private Homebrew tap, GitHub CLI for auth |

## Current Work

See `docs/workflow/scope.md` for the current iteration.

## Critical Files

_TBD — update as the codebase takes shape._

## Data Shapes

_TBD — update as data models are defined._

## Constraints

- Must compile and run on macOS and Linux
- No `unsafe` Rust
- Must work fully offline — no runtime network dependencies
- Keep this file under 200 lines
