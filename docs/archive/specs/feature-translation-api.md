# Technical Specification: Translation API

_Created: 2026-04-15_
_Status: Draft_
_Author: AI-assisted_
_GitHub Issue: #6_

## Overview

Allow users to bring their own API key to fetch Bible translations from external providers at runtime, unlocking access to translations beyond the bundled KJV. This is the first feature that introduces optional network connectivity — the app remains fully functional offline with KJV, but can fetch additional translations when an API key is configured.

## Requirements

### Functional Requirements

1. **FR-1**: User can configure an API key via a config file or environment variable
2. **FR-2**: User can select non-offline translations from the translation picker when an API key is present
3. **FR-3**: Fetched translations are cached locally so repeated reads don't require network
4. **FR-4**: The app remains fully functional offline — KJV is always available without an API key
5. **FR-5**: The translation picker indicates which translations are available offline vs. require API
6. **FR-6**: Errors from the API (network failure, invalid key, rate limit) are displayed gracefully in the status bar

### Non-Functional Requirements

1. **NFR-1**: API calls must not block the TUI event loop — use async or background threads
2. **NFR-2**: Cached translations must load as fast as the bundled KJV
3. **NFR-3**: No `unsafe` Rust
4. **NFR-4**: API key must not be logged, included in error messages, or persisted anywhere except the config file

## Technical Design

### Architecture

A new `api/` module handles external Bible API communication. A local SQLite cache stores fetched chapters so they don't need re-fetching. The existing `db.rs` query functions gain a cache fallback path.

```
src/
├── api/
│   ├── mod.rs          # ApiClient, ApiConfig
│   ├── provider.rs     # Provider trait + implementations
│   └── cache.rs        # Local SQLite cache for fetched translations
```

### Configuration

```rust
// config/api.rs
pub struct ApiConfig {
    pub provider: String,       // e.g., "bible-api" or "api.bible"
    pub api_key: Option<String>,
}
```

Loaded from `providers.json` in the app data directory (`data_dir()` via `directories` crate) or `SELAH_API_KEY` environment variable. The config file is separate from session/bookmarks to avoid accidental sharing.

### Provider Abstraction

```rust
// api/provider.rs
pub trait BibleProvider {
    fn fetch_chapter(
        &self,
        translation: &str,
        book_num: u32,
        chapter: u32,
    ) -> Result<Vec<Verse>, ApiError>;
}
```

Initial implementation targets one public Bible API (to be determined based on licensing and availability). The trait allows adding providers later.

### Cache Layer

A writable SQLite database in the platform data directory stores fetched chapters:

```sql
CREATE TABLE cached_verses (
    translation TEXT NOT NULL,
    book_num INTEGER NOT NULL,
    chapter INTEGER NOT NULL,
    verse INTEGER NOT NULL,
    text TEXT NOT NULL,
    fetched_at INTEGER NOT NULL,
    PRIMARY KEY (translation, book_num, chapter, verse)
);
```

Query flow:
1. Check bundled DB (offline translations)
2. Check cache DB
3. Fetch from API -> store in cache -> return

### Non-Blocking Fetch

The TUI event loop must not block on network. Approach: spawn a `std::thread` for the API call, use a `mpsc::channel` to send results back to the main loop. While fetching, the Scripture panel shows a "Loading..." indicator.

### Translation Picker Changes

The existing `TranslationPickerState` already shows all translations with an `offline` flag. Changes:
- Translations with `offline: false` are selectable when an API key is configured
- A visual indicator shows: `[offline]`, `[cached]`, or `[api]` next to each translation
- Selecting a non-cached API translation triggers a fetch

### Component Design

| Component | File | Responsibility |
|-----------|------|----------------|
| `ApiConfig` | `src/config/api.rs` | Load API key from file or env |
| `BibleProvider` trait | `src/api/provider.rs` | Abstraction for Bible API providers |
| `CacheDb` | `src/api/cache.rs` | Local SQLite cache for fetched verses |
| `ApiClient` | `src/api/mod.rs` | Orchestrates provider + cache |
| `TranslationPickerState` | `src/ui/translation.rs` | Updated to show availability status |
| `App` | `src/app.rs` | Owns `ApiClient`, handles async fetch results |

## Dependencies

| Dependency | Type | Notes |
|------------|------|-------|
| `ureq` | Runtime (optional) | HTTP client for API calls; feature-gated |
| `toml` | Runtime (optional) | Parse API config file |
| `rusqlite` | Runtime | Already in use; reused for cache DB |

### Feature Gate

The API functionality should be behind a cargo feature flag (`api`) so the binary can still be built fully offline with zero network dependencies:

```toml
[features]
default = []
api = ["ureq", "toml"]
```

## Alternatives Considered

### Option A: Bundle additional translations at compile time

Download more SQLite files and embed them.

- **Pros**: Stays fully offline; simple
- **Cons**: Massively increases binary size; licensing issues with many translations; no user customization

### Option B: Runtime API with BYOK (Bring Your Own Key)

- **Pros**: Small binary; user controls which translations; extensible
- **Cons**: Requires network for non-KJV; more complex

### Option C: User downloads translation files manually, app loads from disk

- **Pros**: Offline after download; no API key needed
- **Cons**: Poor UX; file format compatibility issues

### Decision

Option B — runtime API with BYOK. This matches the user story exactly and keeps the binary small. Feature-gating ensures the core offline experience is preserved.

## Security Considerations

- **API key storage**: Config file should have restricted permissions (0600). Warn if permissions are too open
- **API key handling**: Never log, print, or include in error messages
- **HTTPS only**: All API calls must use HTTPS
- **Input validation**: Sanitize API responses before inserting into cache DB
- **No key in binary**: API key is user-provided at runtime, never compiled in

## Testing Strategy

- Unit tests: `config/api.rs` — config loading from file and env var
- Unit tests: `api/cache.rs` — cache insert, lookup, miss
- Unit tests: `BibleProvider` — mock provider for fetch/error scenarios
- Integration tests: Full flow with mock HTTP server — fetch, cache, display
- Manual verification: Translation picker with/without API key; network error display; cache hit after first fetch

## Open Questions

- [ ] Which Bible API provider to target first? (api.bible, bible-api.com, etc.)
- [ ] Should cached translations have a TTL, or persist indefinitely?
- [ ] Should the feature flag be `api` or should it be the default in v0.2.0?
- [ ] How to handle translations that require attribution or have usage restrictions?

## References

- [Translation list](../../src/bible/mod.rs) — `TRANSLATIONS` array with `offline` flag
- [Translation picker](../../src/ui/translation.rs) — current overlay
- [DB module](../../src/bible/db.rs) — query pattern to extend
- GitHub Issue: #6
