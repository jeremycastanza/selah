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
        let url = format!("{API_BASE}/v1/bibles/{version_id}/passages/{usfm}?format=html");
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

/// Strip HTML tags from a string, preserving inner text.
fn strip_html_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }
    result
}

/// Parse YV API HTML response into individual verses.
///
/// The HTML format uses `<span class="yv-v" v="N">` markers to delimit
/// verses. We split on these markers and strip tags from each segment.
pub fn parse_passage_to_verses(
    content: &str,
    book_num: u32,
    chapter: u32,
    translation: &str,
) -> Vec<Verse> {
    let book = book_name(book_num).to_string();
    let mut verses: Vec<Verse> = Vec::new();

    // Split on verse marker pattern: <span class="yv-v" v="N">
    let marker = "class=\"yv-v\" v=\"";
    let mut remaining = content;

    while let Some(pos) = remaining.find(marker) {
        let after_marker = &remaining[pos + marker.len()..];
        let end_quote = match after_marker.find('"') {
            Some(i) => i,
            None => break,
        };
        let verse_num: u32 = match after_marker[..end_quote].parse() {
            Ok(n) => n,
            Err(_) => break,
        };

        // Find start of next verse marker (or end of string) for text extraction
        let text_start = match after_marker[end_quote..].find('>') {
            Some(i) => end_quote + i + 1,
            None => break,
        };

        // Skip the yv-vlbl span that follows: <span class="yv-vlbl">N</span>
        let text_content = &after_marker[text_start..];
        let text_content = if let Some(lbl_start) = text_content.find("class=\"yv-vlbl\">") {
            let after_lbl = &text_content[lbl_start..];
            match after_lbl.find("</span>") {
                Some(i) => &text_content[lbl_start + i + 7..],
                None => text_content,
            }
        } else {
            text_content
        };

        // Find where this verse's text ends (next verse marker or end)
        let text_end = text_content.find(marker).unwrap_or(text_content.len());
        let raw_text = &text_content[..text_end];
        let text = strip_html_tags(raw_text).trim().to_string();

        if !text.is_empty() {
            verses.push(Verse {
                book: book.clone(),
                book_num,
                chapter,
                verse: verse_num,
                text,
                translation: translation.to_uppercase(),
            });
        }

        remaining = if text_end < text_content.len() {
            &text_content[text_end..]
        } else {
            ""
        };
    }

    // Fallback: if no verse markers found, treat entire content as verse 1
    if verses.is_empty() && !content.trim().is_empty() {
        let text = strip_html_tags(content).trim().to_string();
        if !text.is_empty() {
            verses.push(Verse {
                book,
                book_num,
                chapter,
                verse: 1,
                text,
                translation: translation.to_uppercase(),
            });
        }
    }

    verses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_html_verses() {
        let content = r#"<div><div class="p"><span class="yv-v" v="1"></span><span class="yv-vlbl">1</span>In the beginning God created the heavens and the earth. <span class="yv-v" v="2"></span><span class="yv-vlbl">2</span>Now the earth was formless and void.</div></div>"#;
        let verses = parse_passage_to_verses(content, 1, 1, "bsb");
        assert_eq!(verses.len(), 2);
        assert_eq!(verses[0].verse, 1);
        assert_eq!(
            verses[0].text,
            "In the beginning God created the heavens and the earth."
        );
        assert_eq!(verses[1].verse, 2);
        assert_eq!(verses[1].text, "Now the earth was formless and void.");
        assert_eq!(verses[0].book, "Genesis");
        assert_eq!(verses[0].translation, "BSB");
    }

    #[test]
    fn parse_html_single_verse() {
        let content = r#"<div><div class="p"><span class="yv-v" v="16"></span><span class="yv-vlbl">16</span>For God so loved the world that He gave His one and only Son.</div></div>"#;
        let verses = parse_passage_to_verses(content, 43, 3, "bsb");
        assert_eq!(verses.len(), 1);
        assert_eq!(verses[0].verse, 16);
        assert_eq!(verses[0].book, "John");
        assert_eq!(verses[0].chapter, 3);
        assert!(verses[0].text.starts_with("For God so loved"));
    }

    #[test]
    fn parse_html_multiline_paragraph() {
        // Verses that span across paragraph divs
        let content = r#"<div><div class="p"><span class="yv-v" v="5"></span><span class="yv-vlbl">5</span>Jesus answered, "Truly I tell you." <span class="yv-v" v="6"></span><span class="yv-vlbl">6</span>Flesh is born of flesh.</div><div class="p"><span class="yv-v" v="7"></span><span class="yv-vlbl">7</span>Do not be amazed.</div></div>"#;
        let verses = parse_passage_to_verses(content, 43, 3, "bsb");
        assert_eq!(verses.len(), 3);
        assert_eq!(verses[0].verse, 5);
        assert_eq!(verses[1].verse, 6);
        assert_eq!(verses[2].verse, 7);
        assert_eq!(verses[2].text, "Do not be amazed.");
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
