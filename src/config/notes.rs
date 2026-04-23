use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct NoteEntry {
    pub book: String,
    pub chapter: u32,
    pub verse: u32,
    pub text: String,
    pub created_at: u64,
    pub updated_at: u64,
}

fn notes_path() -> Option<PathBuf> {
    let dirs = ProjectDirs::from("", "", "selah")?;
    Some(dirs.data_dir().join("notes.json"))
}

pub fn load() -> Vec<NoteEntry> {
    let Some(path) = notes_path() else {
        return vec![];
    };
    fs::read_to_string(&path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

pub fn save(notes: &[NoteEntry]) {
    let Some(path) = notes_path() else {
        return;
    };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(
        &path,
        serde_json::to_string_pretty(notes).unwrap_or_default(),
    );
}

pub fn find<'a>(
    notes: &'a [NoteEntry],
    book: &str,
    chapter: u32,
    verse: u32,
) -> Option<&'a NoteEntry> {
    notes
        .iter()
        .find(|n| n.book == book && n.chapter == chapter && n.verse == verse)
}

pub fn upsert(notes: &mut Vec<NoteEntry>, book: &str, chapter: u32, verse: u32, text: &str) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    if let Some(existing) = notes
        .iter_mut()
        .find(|n| n.book == book && n.chapter == chapter && n.verse == verse)
    {
        existing.text = text.to_string();
        existing.updated_at = now;
    } else {
        notes.push(NoteEntry {
            book: book.to_string(),
            chapter,
            verse,
            text: text.to_string(),
            created_at: now,
            updated_at: now,
        });
    }
    save(notes);
}

pub fn remove(notes: &mut Vec<NoteEntry>, book: &str, chapter: u32, verse: u32) {
    notes.retain(|n| !(n.book == book && n.chapter == chapter && n.verse == verse));
    save(notes);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(book: &str, chapter: u32, verse: u32, text: &str) -> NoteEntry {
        NoteEntry {
            book: book.to_string(),
            chapter,
            verse,
            text: text.to_string(),
            created_at: 0,
            updated_at: 0,
        }
    }

    #[test]
    fn serde_round_trip() {
        let note = entry("John", 3, 16, "A beloved verse");
        let json = serde_json::to_string(&note).unwrap();
        let loaded: NoteEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.book, "John");
        assert_eq!(loaded.chapter, 3);
        assert_eq!(loaded.verse, 16);
        assert_eq!(loaded.text, "A beloved verse");
    }

    #[test]
    fn upsert_creates_new() {
        let mut notes = vec![];
        upsert(&mut notes, "John", 3, 16, "My note");
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].text, "My note");
    }

    #[test]
    fn upsert_updates_existing() {
        let mut notes = vec![entry("John", 3, 16, "Old note")];
        upsert(&mut notes, "John", 3, 16, "New note");
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].text, "New note");
    }

    #[test]
    fn remove_deletes_by_verse() {
        let mut notes = vec![
            entry("John", 3, 16, "Note 1"),
            entry("Romans", 8, 28, "Note 2"),
        ];
        remove(&mut notes, "John", 3, 16);
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].book, "Romans");
    }

    #[test]
    fn find_returns_matching_note() {
        let notes = vec![entry("John", 3, 16, "Found it")];
        let result = find(&notes, "John", 3, 16);
        assert!(result.is_some());
        assert_eq!(result.unwrap().text, "Found it");
    }

    #[test]
    fn find_returns_none_when_missing() {
        let notes = vec![entry("John", 3, 16, "Found it")];
        let result = find(&notes, "Romans", 8, 28);
        assert!(result.is_none());
    }
}
