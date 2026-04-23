use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum HighlightColor {
    Yellow,
    Green,
    Blue,
    Pink,
    Orange,
}

impl HighlightColor {
    pub fn next(self) -> Option<Self> {
        match self {
            Self::Yellow => Some(Self::Green),
            Self::Green => Some(Self::Blue),
            Self::Blue => Some(Self::Pink),
            Self::Pink => Some(Self::Orange),
            Self::Orange => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Yellow => "yellow",
            Self::Green => "green",
            Self::Blue => "blue",
            Self::Pink => "pink",
            Self::Orange => "orange",
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HighlightEntry {
    pub book: String,
    pub chapter: u32,
    pub verse: u32,
    pub color: HighlightColor,
    pub created_at: u64,
}

pub type HighlightMap = HashMap<(String, u32, u32), HighlightColor>;

fn highlights_path() -> Option<PathBuf> {
    let dirs = ProjectDirs::from("", "", "selah")?;
    Some(dirs.data_dir().join("highlights.json"))
}

pub fn load() -> Vec<HighlightEntry> {
    let Some(path) = highlights_path() else {
        return vec![];
    };
    fs::read_to_string(&path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

pub fn save(highlights: &[HighlightEntry]) {
    let Some(path) = highlights_path() else {
        return;
    };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(
        &path,
        serde_json::to_string_pretty(highlights).unwrap_or_default(),
    );
}

pub fn build_map(highlights: &[HighlightEntry]) -> HighlightMap {
    highlights
        .iter()
        .map(|h| ((h.book.clone(), h.chapter, h.verse), h.color))
        .collect()
}

pub fn toggle(
    highlights: &mut Vec<HighlightEntry>,
    book: &str,
    chapter: u32,
    verse: u32,
) -> Option<HighlightColor> {
    let pos = highlights
        .iter()
        .position(|h| h.book == book && h.chapter == chapter && h.verse == verse);

    match pos {
        Some(idx) => {
            let current = highlights[idx].color;
            match current.next() {
                Some(next_color) => {
                    highlights[idx].color = next_color;
                    save(highlights);
                    Some(next_color)
                }
                None => {
                    highlights.remove(idx);
                    save(highlights);
                    None
                }
            }
        }
        None => {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            highlights.push(HighlightEntry {
                book: book.to_string(),
                chapter,
                verse,
                color: HighlightColor::Yellow,
                created_at: now,
            });
            save(highlights);
            Some(HighlightColor::Yellow)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_next_cycles_through_all() {
        assert_eq!(HighlightColor::Yellow.next(), Some(HighlightColor::Green));
        assert_eq!(HighlightColor::Green.next(), Some(HighlightColor::Blue));
        assert_eq!(HighlightColor::Blue.next(), Some(HighlightColor::Pink));
        assert_eq!(HighlightColor::Pink.next(), Some(HighlightColor::Orange));
        assert_eq!(HighlightColor::Orange.next(), None);
    }

    #[test]
    fn toggle_adds_yellow_when_no_highlight() {
        let mut highlights = vec![];
        let result = toggle(&mut highlights, "John", 3, 16);
        assert_eq!(result, Some(HighlightColor::Yellow));
        assert_eq!(highlights.len(), 1);
        assert_eq!(highlights[0].color, HighlightColor::Yellow);
    }

    #[test]
    fn toggle_advances_color() {
        let mut highlights = vec![HighlightEntry {
            book: "John".to_string(),
            chapter: 3,
            verse: 16,
            color: HighlightColor::Yellow,
            created_at: 0,
        }];
        let result = toggle(&mut highlights, "John", 3, 16);
        assert_eq!(result, Some(HighlightColor::Green));
        assert_eq!(highlights.len(), 1);
    }

    #[test]
    fn toggle_removes_on_orange() {
        let mut highlights = vec![HighlightEntry {
            book: "John".to_string(),
            chapter: 3,
            verse: 16,
            color: HighlightColor::Orange,
            created_at: 0,
        }];
        let result = toggle(&mut highlights, "John", 3, 16);
        assert_eq!(result, None);
        assert!(highlights.is_empty());
    }

    #[test]
    fn build_map_creates_correct_hashmap() {
        let highlights = vec![
            HighlightEntry {
                book: "John".to_string(),
                chapter: 3,
                verse: 16,
                color: HighlightColor::Yellow,
                created_at: 0,
            },
            HighlightEntry {
                book: "Romans".to_string(),
                chapter: 8,
                verse: 28,
                color: HighlightColor::Blue,
                created_at: 0,
            },
        ];
        let map = build_map(&highlights);
        assert_eq!(map.len(), 2);
        assert_eq!(
            map.get(&("John".to_string(), 3, 16)),
            Some(&HighlightColor::Yellow)
        );
        assert_eq!(
            map.get(&("Romans".to_string(), 8, 28)),
            Some(&HighlightColor::Blue)
        );
    }

    #[test]
    fn serde_round_trip() {
        let entry = HighlightEntry {
            book: "Genesis".to_string(),
            chapter: 1,
            verse: 1,
            color: HighlightColor::Pink,
            created_at: 12345,
        };
        let json = serde_json::to_string(&entry).unwrap();
        let loaded: HighlightEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.book, "Genesis");
        assert_eq!(loaded.chapter, 1);
        assert_eq!(loaded.verse, 1);
        assert_eq!(loaded.color, HighlightColor::Pink);
        assert_eq!(loaded.created_at, 12345);
    }
}
