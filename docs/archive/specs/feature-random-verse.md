# Technical Specification: F2 — Random Verse

_Created: 2026-03-26_
_Status: Draft_

## Overview

Display a randomly selected verse from the current translation. Available both as a CLI subcommand and as an in-TUI keybinding.

## Requirements

### Functional

- **FR-1**: `selah random` CLI subcommand prints a random verse to stdout (plain text when piped, TUI verse card when in a TTY)
- **FR-2**: In the TUI browser, pressing `r` navigates to a random verse (updates book/chapter/verse selection, loads chapter, scrolls Scripture to that verse)
- **FR-3**: Random selection is uniform across all verses in the active translation

### Non-Functional

- **NFR-1**: Selection must work fully offline — uses SQLite `ORDER BY RANDOM()`, no external API

## User Interaction Flow

**CLI path:**
1. User runs `selah random` in a piped context → output: `John 3:16 — For God so loved the world...`
2. User runs `selah random` in a TTY → TUI verse card is shown; press `q`/`Esc` to exit

**TUI path:**
1. User is in the browser and presses `r`
2. A random verse is fetched from SQLite
3. `BrowserState` is updated: `selected_book_idx`, `selected_chapter`, `selected_verse`
4. Chapter is loaded; Scripture panel scrolls to the verse
5. Status bar briefly shows `"→ John 3:16"` via `status_flash`

## Technical Design

### Module

`bible/random.rs`:

```rust
pub fn get_random_verse(conn: &Connection, translation: &str) -> rusqlite::Result<Verse> {
    let table = format!("t_{}", translation.to_lowercase());
    let query = format!("SELECT b, c, v, t FROM {} ORDER BY RANDOM() LIMIT 1", table);
    // map row to Verse
}
```

### TUI Integration

In `app.rs` / `handle_key`:
```rust
KeyCode::Char('r') => {
    if let Some(verse) = db.get_random_verse(&state.translation).ok() {
        state.jump_to_verse(verse.book_num, verse.chapter, verse.verse);
        load_chapter();
    }
}
```

`BrowserState::jump_to_verse(book_num, chapter, verse)` updates list states and scroll position.

### CLI Integration

`cli.rs` defines a `random` subcommand. In `main.rs`:
```rust
Commands::Random { translation } => {
    let verse = db::get_random_verse(&conn, &translation)?;
    if stdout_is_tty() {
        render_verse_card(&[verse])?;
    } else {
        println!("{} {}:{} — {}", verse.book, verse.chapter, verse.verse, verse.text);
    }
}
```

`render_verse_card` is a minimal single-verse TUI view (ported from `christ-cli`'s `verse_card.rs`) — shows a centered card with the verse text and reference. Press `q`/`Enter`/`Esc` to exit.

## Dependencies

- `bible/db.rs` — SQLite connection and query
- `ui/browser.rs` — `jump_to_verse` method on `BrowserState`
- F1 (chapter load after jump)

## Testing Strategy

- **Manual**: Run `selah random` multiple times — verify different verses appear
- **Manual**: Press `r` in TUI — verify navigation to a verse in a random location
- **Automated**: Unit test `get_random_verse` returns a valid `Verse` with populated fields
- **Automated**: Test `format!` output matches `"Book Chapter:Verse — Text"` pattern

## References

- `christ-cli` source: `src/main.rs::cmd_random`, `src/ui/verse_card.rs`
