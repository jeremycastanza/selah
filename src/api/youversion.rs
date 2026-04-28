use std::fmt;

use serde::Deserialize;

use crate::bible::books::book_name;
use crate::bible::types::Verse;

#[derive(Debug)]
pub enum ApiError {
    Network(String),
    InvalidKey,
    RateLimited,
    NotFound,
    ServerError(u16),
    ParseError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Network(msg) => write!(f, "Network error: {msg}"),
            Self::InvalidKey => write!(f, "Invalid API key"),
            Self::RateLimited => write!(f, "Rate limited — try again later"),
            Self::NotFound => write!(f, "Resource not found"),
            Self::ServerError(code) => write!(f, "Server error ({code})"),
            Self::ParseError(msg) => write!(f, "Parse error: {msg}"),
        }
    }
}

impl std::error::Error for ApiError {}

#[derive(Debug, Clone, Deserialize)]
pub struct YvVersionsResponse {
    pub data: Vec<YvVersion>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YvVersion {
    pub id: u32,
    pub abbreviation: String,
    pub localized_title: String,
    pub language_tag: String,
    pub copyright: Option<String>,
    #[serde(default)]
    pub books: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YvPassage {
    pub id: String,
    pub content: String,
    pub reference: String,
}

const API_BASE: &str = "https://api.youversion.com";

pub struct YouVersionClient {
    app_key: String,
}

impl YouVersionClient {
    pub fn new(app_key: String) -> Self {
        Self { app_key }
    }

    pub fn get_versions(&self, language: &str) -> Result<Vec<YvVersion>, ApiError> {
        let url = format!("{API_BASE}/v1/bibles?language_ranges[]={language}");
        let response: YvVersionsResponse = self.get_json(&url)?;
        Ok(response.data)
    }

    pub fn get_version(&self, version_id: u32) -> Result<YvVersion, ApiError> {
        let url = format!("{API_BASE}/v1/bibles/{version_id}");
        self.get_json(&url)
    }

    pub fn get_passage(&self, version_id: u32, usfm: &str) -> Result<YvPassage, ApiError> {
        let url = format!("{API_BASE}/v1/bibles/{version_id}/passages/{usfm}?format=text");
        self.get_json(&url)
    }

    fn get_json<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T, ApiError> {
        let response = ureq::get(url)
            .header("X-YVP-App-Key", &self.app_key)
            .header("Accept", "application/json")
            .call()
            .map_err(|e| match e {
                ureq::Error::StatusCode(code) => match code {
                    401 | 403 => ApiError::InvalidKey,
                    404 => ApiError::NotFound,
                    429 => ApiError::RateLimited,
                    _ => ApiError::ServerError(code),
                },
                other => ApiError::Network(other.to_string()),
            })?;

        response
            .into_body()
            .read_json::<T>()
            .map_err(|e| ApiError::ParseError(e.to_string()))
    }
}

pub fn parse_passage_to_verses(
    content: &str,
    book_num: u32,
    chapter: u32,
    translation: &str,
) -> Vec<Verse> {
    let mut verses: Vec<Verse> = Vec::new();
    let book = book_name(book_num).to_string();

    let mut current_verse: Option<u32> = None;
    let mut current_text = String::new();
    let mut chars = content.char_indices().peekable();

    while let Some((i, ch)) = chars.next() {
        if ch == '[' {
            let rest = &content[i + 1..];
            if let Some(end) = rest.find(']') {
                let inside = &rest[..end];
                if let Ok(num) = inside.trim().parse::<u32>() {
                    if let Some(v) = current_verse {
                        let text = current_text.trim().to_string();
                        if !text.is_empty() {
                            verses.push(Verse {
                                book: book.clone(),
                                book_num,
                                chapter,
                                verse: v,
                                text,
                                translation: translation.to_uppercase(),
                            });
                        }
                    }
                    current_verse = Some(num);
                    current_text.clear();
                    for _ in 0..=end {
                        chars.next();
                    }
                    continue;
                }
            }
        }
        if current_verse.is_some() {
            current_text.push(ch);
        }
    }

    if let Some(v) = current_verse {
        let text = current_text.trim().to_string();
        if !text.is_empty() {
            verses.push(Verse {
                book: book.clone(),
                book_num,
                chapter,
                verse: v,
                text,
                translation: translation.to_uppercase(),
            });
        }
    }

    if verses.is_empty() && !content.trim().is_empty() {
        verses.push(Verse {
            book,
            book_num,
            chapter,
            verse: 1,
            text: content.trim().to_string(),
            translation: translation.to_uppercase(),
        });
    }

    verses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bracketed_verses() {
        let content = "[1] In the beginning God created the heavens and the earth. [2] Now the earth was formless and void.";
        let verses = parse_passage_to_verses(content, 1, 1, "kjv");
        assert_eq!(verses.len(), 2);
        assert_eq!(verses[0].verse, 1);
        assert_eq!(
            verses[0].text,
            "In the beginning God created the heavens and the earth."
        );
        assert_eq!(verses[1].verse, 2);
        assert_eq!(verses[1].text, "Now the earth was formless and void.");
        assert_eq!(verses[0].book, "Genesis");
        assert_eq!(verses[0].translation, "KJV");
    }

    #[test]
    fn parse_single_verse() {
        let content = "[16] For God so loved the world that he gave his one and only Son.";
        let verses = parse_passage_to_verses(content, 43, 3, "niv");
        assert_eq!(verses.len(), 1);
        assert_eq!(verses[0].verse, 16);
        assert_eq!(verses[0].book, "John");
        assert_eq!(verses[0].chapter, 3);
        assert!(verses[0].text.starts_with("For God so loved"));
    }

    #[test]
    fn parse_no_markers_fallback() {
        let content = "Some plain text without markers";
        let verses = parse_passage_to_verses(content, 1, 1, "kjv");
        assert_eq!(verses.len(), 1);
        assert_eq!(verses[0].verse, 1);
        assert_eq!(verses[0].text, "Some plain text without markers");
    }

    #[test]
    fn parse_empty_content() {
        let verses = parse_passage_to_verses("", 1, 1, "kjv");
        assert!(verses.is_empty());
    }

    #[test]
    fn api_error_display() {
        assert_eq!(
            ApiError::Network("timeout".into()).to_string(),
            "Network error: timeout"
        );
        assert_eq!(ApiError::InvalidKey.to_string(), "Invalid API key");
        assert_eq!(
            ApiError::RateLimited.to_string(),
            "Rate limited — try again later"
        );
        assert_eq!(ApiError::NotFound.to_string(), "Resource not found");
        assert_eq!(ApiError::ServerError(500).to_string(), "Server error (500)");
        assert_eq!(
            ApiError::ParseError("bad json".into()).to_string(),
            "Parse error: bad json"
        );
    }

    #[test]
    #[ignore]
    fn live_api_fetch_versions() {
        let key = std::env::var("SELAH_YVP_APP_KEY").expect("Set SELAH_YVP_APP_KEY");
        let client = YouVersionClient::new(key);
        let versions = client
            .get_versions("en")
            .expect("Should fetch English versions");
        assert!(!versions.is_empty());
        eprintln!("Got {} versions", versions.len());
        for v in &versions {
            eprintln!("  {} (id={}) - {}", v.abbreviation, v.id, v.localized_title);
        }
    }

    #[test]
    #[ignore]
    fn live_api_fetch_passage() {
        let key = std::env::var("SELAH_YVP_APP_KEY").expect("Set SELAH_YVP_APP_KEY");
        let client = YouVersionClient::new(key);
        let passage = client
            .get_passage(3034, "JHN.3.16")
            .expect("Should fetch John 3:16");
        assert!(!passage.content.is_empty());
        assert_eq!(passage.reference, "John 3:16");
    }
}
