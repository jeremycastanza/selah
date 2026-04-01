# CLAUDE.md — Selah

A fully offline terminal Bible reader built in Rust.

@docs/workflow/context.md

## Current Scope

@docs/workflow/scope.md

## Project Rules

- No `unsafe` Rust anywhere
- No network requests at runtime — all Bible data is bundled in the binary
- Follow the phased build plan in `docs/plans/v0.1.0-implementation.md`
- After every change: `cargo clippy -- -D warnings` and `cargo fmt --check` must be clean
- Never commit directly to `main` — branch + PR

## Documentation

This project uses a unified documentation system in `docs/`:

| Directory | Purpose | When to Read |
|-----------|---------|--------------|
| `docs/workflow/context.md` | Project architecture, patterns, tech stack | Before making suggestions or writing code |
| `docs/workflow/scope.md` | Current iteration objectives and constraints | Before starting any task |
| `docs/workflow/workflow.md` | Build methodology and collaboration process | When planning multi-step work |
| `docs/plans/` | Active implementation plans | When a plan exists for the current task |
| `docs/specs/` | Technical specifications | When implementing a specified feature |
| `docs/rules/` | Project-specific AI behavioral rules | Always (add as @imports in Project Rules) |
| `docs/technical/` | Domain reference (known issues, deployment) | When working in that domain |
| `docs/architecture.md` | System design overview | Before proposing structural changes |

**Progressive discovery:** Start with `context.md` and `scope.md` (loaded above). Read other docs on demand — only when the task requires that specific context.

## Prerequisites

- Rust toolchain (`rustup`) — edition 2024
- No other runtime dependencies; `rusqlite` bundles SQLite from source

## Quick Reference

```bash
cargo build                    # debug build
cargo run                      # run the TUI
cargo run -- random            # print a random verse to stdout
cargo run -- --no-banner       # skip splash screen
cargo test                     # run all unit tests
cargo clippy -- -D warnings    # lint (must be clean)
cargo fmt --check              # format check
cargo fmt                      # auto-format
cargo build --release          # optimized binary → target/release/selah
```
