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
