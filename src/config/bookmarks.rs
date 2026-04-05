use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BookmarkEntry {
    pub book: String,
    pub chapter: u32,
    pub verse: u32,
    pub snippet: Option<String>,
    pub note: Option<String>,
    pub created_at: u64,
}

fn bookmarks_path() -> Option<PathBuf> {
    let dirs = ProjectDirs::from("", "", "selah")?;
    Some(dirs.data_dir().join("bookmarks.json"))
}

pub fn load() -> Vec<BookmarkEntry> {
    let Some(path) = bookmarks_path() else {
        return vec![];
    };
    fs::read_to_string(&path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

pub fn save(bookmarks: &[BookmarkEntry]) {
    let Some(path) = bookmarks_path() else {
        return;
    };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(
        &path,
        serde_json::to_string_pretty(bookmarks).unwrap_or_default(),
    );
}

pub fn add(bookmarks: &mut Vec<BookmarkEntry>, entry: BookmarkEntry) {
    let exists = bookmarks
        .iter()
        .any(|b| b.book == entry.book && b.chapter == entry.chapter && b.verse == entry.verse);
    if !exists {
        bookmarks.push(entry);
        save(bookmarks);
    }
}

pub fn remove(bookmarks: &mut Vec<BookmarkEntry>, index: usize) {
    if index < bookmarks.len() {
        bookmarks.remove(index);
        save(bookmarks);
    }
}

pub fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(book: &str, chapter: u32, verse: u32) -> BookmarkEntry {
        BookmarkEntry {
            book: book.to_string(),
            chapter,
            verse,
            snippet: Some("test snippet".to_string()),
            note: None,
            created_at: 0,
        }
    }

    #[test]
    fn add_deduplicates() {
        let mut bookmarks = vec![];
        add(&mut bookmarks, entry("John", 3, 16));
        add(&mut bookmarks, entry("John", 3, 16));
        assert_eq!(bookmarks.len(), 1);
    }

    #[test]
    fn add_different_verses_keeps_both() {
        let mut bookmarks = vec![];
        add(&mut bookmarks, entry("John", 3, 16));
        add(&mut bookmarks, entry("John", 3, 17));
        assert_eq!(bookmarks.len(), 2);
    }

    #[test]
    fn remove_by_index() {
        let mut bookmarks = vec![entry("John", 3, 16), entry("Romans", 8, 28)];
        remove(&mut bookmarks, 0);
        assert_eq!(bookmarks.len(), 1);
        assert_eq!(bookmarks[0].book, "Romans");
    }

    #[test]
    fn serde_round_trip() {
        let bookmarks = vec![entry("Genesis", 1, 1)];
        let json = serde_json::to_string_pretty(&bookmarks).unwrap();
        let loaded: Vec<BookmarkEntry> = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].book, "Genesis");
        assert_eq!(loaded[0].chapter, 1);
        assert_eq!(loaded[0].verse, 1);
    }
}
