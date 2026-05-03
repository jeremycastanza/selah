# Current Scope

_v0.3.0 — Translation API Integration_
_Status: In Progress — Phases 1–3 complete, Phases 4–6 remaining_
_GitHub Issue: #6_
_Plan: `docs/plans/v0.3.0-implementation.md`_

## Objective

Integrate the YouVersion Platform API so users can read Bible translations beyond the bundled KJV. The app stays fully offline for KJV but can fetch additional translations when a YVP app key is configured.

## Completed

- **Phase 1**: USFM alignment — `BookInfo.usfm` field + mapping utilities
- **Phase 2**: Provider config model — `config/providers.rs` + TOML persistence + env var fallback
- **Phase 3**: YouVersion API client — HTTP client + passage parser, feature-gated behind `api`

## Remaining

- **Phase 4**: Cache layer — SQLite cache with 90-day TTL
- **Phase 5**: Data layer integration — Unified resolver (bundled → cache → API), background fetch
- **Phase 6**: UI integration — Settings overlay, translation picker updates, loading states

## Will NOT Build

- OAuth / Sign In With YouVersion
- Syncing highlights/bookmarks to YouVersion
- Translation search/discovery UI
- Multiple simultaneous providers
- FTS5 search on API-fetched translations

## Previous Release

v0.2.0 — Released 2026-04-27. Plan archived at `docs/archive/plans/v0.2.0-implementation.md`.
