# Technical Specification: Contributing Documentation

_Created: 2026-04-15_
_Status: Draft_
_Author: AI-assisted_
_GitHub Issue: #1_

## Overview

Create developer onboarding documentation so contributors can build, test, and submit changes to Selah. This is a documentation-only deliverable with no code changes.

## Requirements

### Functional Requirements

1. **FR-1**: A `CONTRIBUTING.md` file in the repo root explains how to set up the development environment
2. **FR-2**: Document the prerequisite toolchain (Rust edition 2024, rustup)
3. **FR-3**: Document build, test, lint, and format commands
4. **FR-4**: Document the branching and PR workflow (conventional commits, no direct main commits)
5. **FR-5**: Document the project structure at a high level (what lives where)
6. **FR-6**: Document the documentation system (`docs/` directory layout and purpose)

### Non-Functional Requirements

1. **NFR-1**: Keep the document concise — under 200 lines
2. **NFR-2**: Use standard GitHub `CONTRIBUTING.md` conventions so it surfaces automatically in the GitHub UI

## Technical Design

### File Structure

```
CONTRIBUTING.md     # Root-level, GitHub-recognized location
```

### Sections

1. **Prerequisites** — Rust toolchain, edition, platform support (macOS/Linux)
2. **Getting Started** — Clone, build, run
3. **Development Commands** — build, test, clippy, fmt, release build
4. **Project Structure** — Brief directory map (reference `docs/workflow/context.md` for details)
5. **Making Changes** — Branch naming, conventional commits, PR process
6. **Code Standards** — No `unsafe`, no runtime network, clippy + fmt must pass
7. **Documentation** — How the `docs/` system works, when to update docs

### Content Sources

Most content already exists across `CLAUDE.md`, `docs/workflow/context.md`, and `docs/workflow/workflow.md`. The `CONTRIBUTING.md` consolidates this for human developers who won't read AI-facing files.

## Dependencies

None — documentation only.

## Alternatives Considered

### Option A: Single CONTRIBUTING.md

- **Pros**: Standard location; GitHub surfaces it on issue/PR pages; one file to maintain
- **Cons**: Could get long

### Option B: CONTRIBUTING.md + docs/development-guide.md

- **Pros**: Keeps root file short, detailed guide in docs/
- **Cons**: Extra file; contributors might miss the detailed guide

### Decision

Option A — single `CONTRIBUTING.md`. The project is small enough that one concise file covers everything. Link to `docs/` for deeper context.

## Security Considerations

None — documentation only.

## Testing Strategy

- Manual review: Verify a fresh clone + documented steps produce a working build
- Link check: Verify internal links to docs/ files resolve correctly

## Resolved Questions

- **Code of conduct?** — No, not needed for v0.2.0.
- **Issue/PR templates?** — No, not needed for v0.2.0.

## References

- [CLAUDE.md](../../.claude/CLAUDE.md) — existing project instructions
- [Project context](../../docs/workflow/context.md) — architecture overview
- GitHub Issue: #1
