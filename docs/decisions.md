# Decisions

Rolling log of Architecture Decision Records (ADRs). Auto-archives when reaching 10 entries.

---

## ADR-001: Rust as the Implementation Language

_Date: 2026-03-25_
_Status: Accepted_

### Context

Two source forks exist: `christ` (Rust) and `bible` (JavaScript). Selah needs to combine features from both into a single product. A language choice must be made.

### Decision

Selah is implemented entirely in Rust. Features from the `bible` (JavaScript) fork will be ported to Rust rather than embedded via Node.js or a JS runtime.

### Consequences

**Positive:**
- Single language simplifies the build, toolchain, and contributor experience
- Rust provides excellent performance and a small binary suitable for CLI distribution
- The `christ` fork provides an existing Rust foundation to build from
- Aligns with the Homebrew tap distribution model (single compiled binary)

**Negative:**
- JS features from `bible` fork require porting effort rather than direct reuse
- Rust has a steeper learning curve for contributors unfamiliar with it

### Alternatives Considered

1. **Keep JavaScript for the `bible` features** — Rejected; would require bundling Node.js or a separate runtime, complicating distribution
2. **Rewrite in Go** — Rejected; the `christ` fork is already in Rust, no reason to abandon that investment

---

## ADR-002: macOS and Linux as Target Platforms

_Date: 2026-03-25_
_Status: Accepted_

### Context

Selah targets developers who live in the terminal. A platform scope decision is needed to guide CI, testing, and distribution work.

### Decision

Selah officially supports macOS and Linux. Windows is not a target platform.

### Consequences

**Positive:**
- Simplifies CI/CD — only two platform builds needed
- Homebrew tap distribution works natively on both targets
- Terminal/TUI behavior is more consistent across macOS and Linux

**Negative:**
- Windows users cannot run Selah without WSL

### Alternatives Considered

1. **Support Windows via WSL guidance** — May be added informally later, but not an active target
2. **macOS only** — Rejected; Linux is a primary terminal developer platform

---

## ADR-003: Private Homebrew Tap for Distribution

_Date: 2026-03-25_
_Status: Accepted_

### Context

Selah is a personal project targeting a specific audience. A distribution mechanism is needed that supports macOS and Linux, is easy to install, and doesn't require publishing to a public package registry.

### Decision

Selah is distributed via a private Homebrew tap hosted on GitHub. GitHub CLI (`gh`) is used for authentication, mirroring the pattern established by the lexicon-dex CLI.

### Consequences

**Positive:**
- Familiar install UX for developers (`brew install`)
- Private tap keeps distribution controlled without a public registry
- GitHub Actions can automate formula updates on release
- Consistent with the existing lexicon-dex tooling pattern

**Negative:**
- Users must have Homebrew and `gh` CLI installed
- Linux users need Homebrew for Linux (or an alternative install path)

### Alternatives Considered

1. **crates.io** — Rejected; public registry, and `cargo install` is less ergonomic for end users
2. **Direct binary download via GitHub Releases** — May serve as a fallback install path, but not the primary method

---

## ADR-004: Unified Codebase Over Maintaining Two Forks

_Date: 2026-03-25_
_Status: Accepted_

### Context

Two fork projects (`christ` and `bible`) exist with overlapping but complementary feature sets. Maintaining both long-term creates duplication of effort and a fragmented user experience.

### Decision

Selah is a new, unified project that supersedes both forks. It is not a fork of either — it is a clean codebase that incorporates the desired features from each. Both source forks are treated as references, not dependencies.

### Consequences

**Positive:**
- Single codebase to maintain going forward
- Coherent UX — no feature disparity between two separate tools
- Freedom to redesign architecture without being constrained by either fork's structure

**Negative:**
- Features must be deliberately ported; nothing is inherited automatically
- Initial build effort is higher than extending one of the existing forks

### Alternatives Considered

1. **Fork `christ` and add `bible` features** — Rejected; would carry over `christ`'s structural decisions and make it harder to cleanly integrate `bible`'s UX patterns
2. **Fork `bible` and port to Rust** — Rejected; same problem in reverse, and `bible` is JavaScript-first

---

## ADR-005: Ratatui as the TUI Framework

_Date: 2026-03-25_
_Status: Accepted_

### Context

A TUI rendering framework is needed. The `christ` fork already uses Ratatui 0.29. The `bible` fork uses `blessed` (Node.js), which cannot be reused in a Rust project. Options evaluated: Ratatui, Cursive.

### Decision

Selah uses Ratatui with the crossterm backend. This continues the foundation already established in the `christ` fork.

### Consequences

**Positive:**
- Largest Rust TUI ecosystem (19k+ stars, 2,800+ dependent crates)
- Direct reuse of `christ`'s existing Ratatui rendering code as a starting point
- Crossterm provides mouse event support on both macOS and Linux
- Abundant examples of content-browsing apps (pagers, readers) in the Ratatui ecosystem
- Precise constraint-based layout system suits the 4-panel design

**Negative:**
- No built-in widget state management — scroll positions, selections, and focus must be managed manually in app state
- Mouse support requires manual event dispatch (no click-on-widget abstraction)

### Alternatives Considered

1. **Cursive** — Built-in mouse and event loop handling, but smaller ecosystem, slower maintenance, and less layout flexibility. The retained-mode widget tree would fight against a custom 4-panel reader layout.

---

## ADR-006: SQLite (rusqlite bundled) for Bible Data

_Date: 2026-03-25_
_Status: Accepted_

### Context

Selah must support multiple Bible translations, full-text search, and fully offline operation. A data storage approach is needed that works as a single distributed binary. The `christ` fork embeds KJV as JSON via `include_str!()` and fetches all other translations from the bolls.life API at runtime — making it online-dependent for multi-version support. The `bible-tui` fork uses a flat KJV JSON file with no search capability.

### Decision

Selah embeds an SQLite database compiled directly into the binary using `rusqlite` with the `bundled` feature. Bible data is sourced from `scrollmapper/bible_databases` (MIT licensed, 140+ translations). SQLite's FTS5 extension handles full-text search.

### Consequences

**Positive:**
- Fully offline — no runtime network dependency for any translation
- FTS5 full-text search with BM25 ranking, built into SQLite — no custom indexing needed
- Multiple translations in a single file (translation_id column or separate tables)
- `rusqlite` with `bundled` compiles SQLite from source — zero system dependency
- Fast indexed lookup by book/chapter/verse (microsecond range)
- Extensible schema: cross-references, Strong's numbers, morphology can be added later

**Negative:**
- Binary size increases with bundled translations (~3–7 MB per translation)
- `bundled` feature compiles SQLite from C source — adds to build time
- More setup than flat JSON for simple single-translation access

### Alternatives Considered

1. **Flat JSON + `include_str!()`** (current `christ` approach for KJV) — Fast to implement, but no efficient search and multi-version support bloats binary proportionally with no query capability
2. **Runtime API (bolls.life)** (current `christ` approach for non-KJV) — Rejected; violates the offline-first requirement
3. **SWORD Project** — Rejected; C++ library with no Rust bindings, high integration cost

---

## ADR-007: Notes Independent of Bookmarks

_Date: 2026-04-23_
_Status: Accepted_

### Context

v0.2.0 adds per-verse notes. `BookmarkEntry` already carries an unused `note: Option<String>` field. Options: reuse that field (notes live on bookmarks) or create a separate `notes.json` with its own entry type.

### Decision

Notes use a separate `notes.json` file and a dedicated `NoteEntry` struct. The existing `BookmarkEntry.note` field remains available for brief bookmark labels but is not the primary notes storage.

### Consequences

**Positive:**
- Users can annotate a verse without bookmarking it
- Notes and bookmarks have independent lifecycles — deleting one does not affect the other
- Separate list overlays (`B` for bookmarks, `N` for notes) stay focused
- Simpler per-domain CRUD functions

**Negative:**
- Two JSON files instead of one; a verse with both a bookmark and a note has data in two places
- Slight duplication of `(book, chapter, verse)` identity across files

### Alternatives Considered

1. **Reuse `BookmarkEntry.note`** — Rejected; forces users to bookmark before annotating, and blurs the UX between "saved location" and "study insight"

---

## ADR-008: JSON Files for All User Data

_Date: 2026-04-23_
_Status: Accepted_

### Context

v0.2.0 introduces highlights and notes as new persistent data. Bible data already lives in a bundled SQLite database (ADR-006). A decision is needed on whether to extend that database with writable tables or continue the JSON-file pattern used by bookmarks and session state.

### Decision

Highlights and notes follow the existing JSON-file pattern (`highlights.json`, `notes.json`) in the platform data directory via `directories::ProjectDirs`. The bundled SQLite database remains read-only Bible content.

### Consequences

**Positive:**
- Consistency with the existing bookmarks/session persistence pattern
- Human-readable files that users can inspect, back up, or sync manually
- No schema migrations — `#[serde(default)]` handles backward-compatible field additions
- O(1) lookup via `HashMap` built at load time for highlights; acceptable O(n) scan for notes at expected scale (< 10k entries)

**Negative:**
- Full file rewrite on every mutation — acceptable at current scale, not at 100k+ entries
- No transactional guarantees across files (e.g., deleting a verse's bookmark and note is two separate writes)

### Alternatives Considered

1. **Writable SQLite tables alongside the bundled Bible data** — Rejected; mixing read-only bundled data with user-writable data in the same file complicates distribution and upgrade flows
2. **Separate user SQLite database** — Rejected; overkill for the current data scale and adds a migration system requirement

---

## ADR-009: Multi-Line Note Editor with Soft Word Wrap

_Date: 2026-04-23_
_Status: Accepted_

### Context

Study notes need more than a single line. Options: reuse the search box (single line), build a full multi-line text editor, or embed an external editor via `$EDITOR`.

### Decision

A custom multi-line text editor overlay is implemented in `src/ui/notes.rs`. `Enter` inserts a newline; `Ctrl+S` saves; `Esc` cancels. The editor tracks `(cursor_row, cursor_col)` against a `Vec<String>` buffer. Long lines soft-wrap visually at the editor width — no horizontal scroll, no hard-wrap inserted into the buffer. Up/Down arrows navigate visual (wrapped) lines, not logical lines. Vertical scroll activates when content exceeds the visible area.

### Consequences

**Positive:**
- Paragraph-length study reflections possible without leaving the TUI
- Fully offline, no `$EDITOR` shell-out
- Soft wrap preserves original logical lines on save — no spurious line breaks in the stored text
- Visual-line cursor navigation matches user expectation in wrapped editors

**Negative:**
- Custom editor logic: cursor math, wrap math, scroll offset, line merging on backspace at boundaries
- Width-dependent cursor state requires stashing the last rendered width on `NoteEditorState` so key handlers can compute visual-line movement

### Alternatives Considered

1. **Single-line notes** — Rejected; too limiting for study use
2. **Shell out to `$EDITOR`** — Rejected; breaks the offline-first single-binary UX and has no clean path in a TUI event loop
3. **Hard-wrap at editor width (insert real newlines)** — Rejected; corrupts the saved text with presentational line breaks tied to one terminal width

---

## ADR-010: Highlight Visibility Toggle

_Date: 2026-04-23_
_Status: Accepted_

### Context

Highlighted verse backgrounds can clutter the reading view when many verses in a chapter are highlighted. Users may want a clean view without deleting their highlights.

### Decision

A global visibility toggle (`g`) hides all highlight backgrounds without mutating the highlight data. The state (`highlights_visible: bool`) persists in `SessionState` via `#[serde(default = "default_true")]`.

### Consequences

**Positive:**
- Users get a clean reading view on demand
- No destructive operation required to declutter
- State persists across restarts

**Negative:**
- One more piece of session state to maintain

### Alternatives Considered

1. **Per-color visibility toggles** — Rejected; over-engineered for current needs
2. **Delete and re-add highlights** — Rejected; destructive and loses color choices

---

## ADR-011: Browse Overlays for Highlights and Notes

_Date: 2026-04-23_
_Status: Accepted_

### Context

As highlights and notes accumulate, users need a way to find and revisit annotated verses without scrolling every chapter. The Bookmarks overlay already provides this pattern.

### Decision

Two new overlays mirror the Bookmarks overlay: Highlights List (`G`) and Notes List (`N`). Same centered-modal style, same keybindings (`j`/`k` navigate, `Enter` jumps, `d` deletes, `Esc` closes). Entries show the verse reference plus metadata (color label for highlights, first-line preview for notes).

### Consequences

**Positive:**
- Consistent UX across all three browse overlays
- Fast navigation to any annotated verse
- Deletion from the list keeps Scripture panel in sync via the same data mutations used elsewhere

**Negative:**
- Three near-identical overlay implementations — some duplicated list-rendering logic. Acceptable at current scale; candidate for a shared abstraction if a fourth overlay of this shape is added.

### Alternatives Considered

1. **Single unified "annotations" overlay** — Rejected; mixing highlights and notes in one list obscures type-specific actions and visuals
2. **Inline chapter-level jump markers only** — Rejected; doesn't scale across books
