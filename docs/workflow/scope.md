# Current Scope

_Iteration started: 2026-03-25_

## Objective

Combine the best features of the `christ` (Rust) and `bible` (JavaScript) fork projects into a single unified Rust TUI called Selah.

## Success Criteria

From the `bible` fork:
- [x] Clean TUI interface
- [x] Random verse option
- [x] Mouse click interaction
- [ ] Bookmark functionality _(Phase 7)_

From the `christ` fork:
- [ ] Robust splash screen _(Phase 8 — banner is placeholder)_
- [x] Theme toggle
- [x] Search functionality
- [ ] Bible version toggling _(Phase 7)_

## Constraints

- Written entirely in Rust — port any JS features from `bible` fork rather than embedding Node
- Must compile and run on macOS and Linux
- No `unsafe` Rust
- Must work fully offline — all Bible data bundled with the binary, no runtime network requests

## We Will NOT Build

- No web interface
- No cloud sync
- No Windows support (not a target platform)

## Current Plan

`docs/plans/v0.1.0-implementation.md` — Phases 1–6 complete, Phase 7 is next.

## Notes

- Source forks: `christ` (Rust) and `bible` (JavaScript)
- Selah is the unified successor — not a fork of either, but a new project incorporating the best of both
