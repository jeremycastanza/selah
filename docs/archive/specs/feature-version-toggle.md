# Technical Specification: F8 — Bible Version Toggling

_Created: 2026-03-26_
_Status: Draft_

## Overview

Users can switch between available Bible translations via a picker overlay. In v1, only KJV is bundled. The UI and data model support additional translations; non-bundled entries are shown as unavailable until more translations are added.

## Requirements

### Functional

- **FR-1**: Press `v` in browser mode to open the translation picker overlay
- **FR-2**: The picker lists available translations grouped by language
- **FR-3**: Bundled (offline) translations are selectable; unavailable translations are shown grayed out
- **FR-4**: `↑/↓` or `j/k` navigate the list; `Enter` selects; `Esc` or `v` dismisses without change
- **FR-5**: On selection, the current book/chapter is reloaded from the new translation
- **FR-6**: The active translation is shown in the status bar
- **FR-7**: The selected translation persists across restarts (stored in session state)

### Non-Functional

- **NFR-1**: In v1, only KJV is selectable; all other entries display as unavailable
- **NFR-2**: The data model is extensible — adding a translation requires only adding its SQLite table and setting `offline: true` in the `TRANSLATIONS` array

## User Interaction Flow

1. User presses `v` — translation picker overlay opens (right-aligned modal)
2. List shows translations grouped by language (e.g., English, Español, Français...)
3. v1: only "King James Version (KJV)" is selectable; others are grayed out with `[offline soon]` or similar label
4. User presses `j/k` to navigate, `Enter` to select
5. Overlay closes; browser reloads current book/chapter in the selected translation
6. Status bar shows the new translation code

## Technical Design

### Translation Metadata

Static array in `ui/browser.rs` or `bible/mod.rs`, ported from `christ-cli`:

```rust
pub struct TranslationInfo {
    pub code: &'static str,
    pub name: &'static str,
    pub lang: &'static str,
    pub offline: bool,  // true = bundled in SQLite, selectable in v1
}

pub const TRANSLATIONS: &[TranslationInfo] = &[
    TranslationInfo { code: "KJV", name: "King James Version", lang: "English", offline: true },
    TranslationInfo { code: "WEB", name: "World English Bible", lang: "English", offline: false },
    // ... (ported from christ-cli browser.rs, ~30 entries)
];
```

### Picker State

```rust
pub struct TranslationPickerState {
    pub list_state: ListState,
}
```

Held in `BrowserState.overlay = Some(OverlayKind::Translation(TranslationPickerState))`.

### Overlay Rendering

Right-aligned modal, ~50% width, full height of browser area. Uses `Clear` + `Block`. List items:
- Selectable (offline): normal text, selectable via list state
- Non-selectable (not yet offline): `text_muted` color, `[soon]` suffix
- Language group headers: `text_dim`, non-selectable separators

### Translation Switch

On `Enter` with a selectable translation:

```rust
state.translation = selected_code.to_string();
state.overlay = None;
load_chapter(); // reload BrowserState.current_chapter from new translation
```

Chapter load queries `t_{selected_code_lowercase}` table. If the table doesn't exist in the embedded db (not yet bundled), it falls back gracefully with an error message in the status bar.

### Status Bar

Status bar always shows the active translation code: `KJV | Theme: Slate`.

### Persistence

`SessionState.translation: String` stores the active code. Defaults to `"KJV"`.

## Dependencies

- F1 (overlay renders on top of browser; chapter reload uses `BrowserState` load method)
- `bible/db.rs` — chapter query with translation-specific table name
- `config/session.rs` — `translation` field in `SessionState`

## Testing Strategy

- **Manual**: Press `v` — verify overlay opens and KJV is shown as selectable
- **Manual**: Select KJV (only option in v1) — verify overlay closes and translation stays KJV
- **Manual**: Press `Esc` without selecting — verify translation is unchanged
- **Manual**: Quit and relaunch — verify KJV is still the active translation
- **Automated**: Unit test `TRANSLATIONS` array contains at least KJV with `offline: true`
- **Automated**: Unit test that selecting a non-offline translation does not trigger a chapter load (graceful no-op in v1)

## References

- `christ-cli` source: `src/ui/browser.rs` (`TRANSLATIONS` array and picker rendering — direct reference)
- `christ-cli` source: `src/app.rs` (picker key handling)
