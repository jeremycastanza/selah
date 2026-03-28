use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::ui::theme::ThemeName;

#[derive(Serialize, Deserialize)]
pub struct SessionState {
    pub book_index: usize,
    pub chapter: u32,
    pub scroll_position: u16,
    pub active_panel: u8,
    pub theme: ThemeName,
    pub translation: String,
}

impl Default for SessionState {
    fn default() -> Self {
        Self {
            book_index: 0,
            chapter: 1,
            scroll_position: 0,
            active_panel: 0,
            theme: ThemeName::default(),
            translation: "KJV".to_string(),
        }
    }
}

fn session_path() -> Option<PathBuf> {
    let dirs = ProjectDirs::from("", "", "selah")?;
    Some(dirs.data_dir().join("session.json"))
}

pub fn load() -> SessionState {
    let Some(path) = session_path() else {
        return SessionState::default();
    };
    fs::read_to_string(&path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

pub fn save(state: &SessionState) {
    let Some(path) = session_path() else {
        return;
    };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(
        &path,
        serde_json::to_string_pretty(state).unwrap_or_default(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_state_serde_round_trip() {
        let state = SessionState {
            book_index: 42,
            chapter: 3,
            scroll_position: 10,
            active_panel: 2,
            theme: ThemeName::Parchment,
            translation: "KJV".to_string(),
        };
        let json = serde_json::to_string(&state).unwrap();
        let deserialized: SessionState = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.book_index, 42);
        assert_eq!(deserialized.chapter, 3);
        assert_eq!(deserialized.scroll_position, 10);
        assert_eq!(deserialized.active_panel, 2);
        assert_eq!(deserialized.theme, ThemeName::Parchment);
        assert_eq!(deserialized.translation, "KJV");
    }
}
