use std::sync::mpsc;

use rusqlite::Connection;

use crate::bible::TRANSLATIONS;
use crate::bible::db;
use crate::bible::types::Verse;

pub enum FetchResult {
    Ready(Vec<Verse>),
    Loading,
    Error(String),
}

pub struct ResolveContext<'a> {
    pub conn: &'a Connection,
    pub resolver: &'a mut ChapterResolver,
    #[cfg(feature = "api")]
    pub cache: Option<&'a crate::api::cache::CacheDb>,
    #[cfg(feature = "api")]
    pub providers: &'a crate::config::providers::ProvidersConfig,
}

struct PendingFetch {
    receiver: mpsc::Receiver<Result<Vec<Verse>, String>>,
    book_idx: usize,
    chapter: u32,
}

pub struct ChapterResolver {
    pending: Option<PendingFetch>,
}

impl ChapterResolver {
    pub fn new() -> Self {
        Self { pending: None }
    }

    pub fn poll(&mut self) -> Option<(usize, u32, FetchResult)> {
        let pending = self.pending.as_ref()?;
        match pending.receiver.try_recv() {
            Ok(Ok(verses)) => {
                let (idx, ch) = (pending.book_idx, pending.chapter);
                self.pending = None;
                Some((idx, ch, FetchResult::Ready(verses)))
            }
            Ok(Err(msg)) => {
                let (idx, ch) = (pending.book_idx, pending.chapter);
                self.pending = None;
                Some((idx, ch, FetchResult::Error(msg)))
            }
            Err(mpsc::TryRecvError::Empty) => None,
            Err(mpsc::TryRecvError::Disconnected) => {
                self.pending = None;
                Some((0, 0, FetchResult::Error("Fetch failed".to_string())))
            }
        }
    }

    pub fn is_loading(&self) -> bool {
        self.pending.is_some()
    }
}

impl<'a> ResolveContext<'a> {
    pub fn resolve(&mut self, translation: &str, book_num: u32, chapter: u32) -> FetchResult {
        let is_bundled = TRANSLATIONS
            .iter()
            .any(|t| t.code.eq_ignore_ascii_case(translation) && t.offline);

        if is_bundled {
            return FetchResult::Ready(db::get_chapter(self.conn, translation, book_num, chapter));
        }

        #[cfg(feature = "api")]
        {
            self.resolve_api(translation, book_num, chapter)
        }

        #[cfg(not(feature = "api"))]
        FetchResult::Error("API feature not enabled".to_string())
    }

    #[cfg(feature = "api")]
    fn resolve_api(&mut self, translation: &str, book_num: u32, chapter: u32) -> FetchResult {
        let cache = match self.cache {
            Some(c) => c,
            None => return FetchResult::Error("Cache not available".to_string()),
        };

        let versions = cache.get_versions();
        let version_id = match versions
            .iter()
            .find(|v| v.abbreviation.eq_ignore_ascii_case(translation))
        {
            Some(v) => v.version_id,
            None => {
                return FetchResult::Error(format!(
                    "Translation '{translation}' not found — sync versions first"
                ));
            }
        };

        if let Some(verses) = cache.get_chapter(version_id, book_num, chapter) {
            return FetchResult::Ready(verses);
        }

        let api_key = match self.providers.youversion_key() {
            Some(k) => k.to_string(),
            None => return FetchResult::Error("No API key configured".to_string()),
        };

        let usfm = format!("{}.{}", crate::bible::books::book_usfm(book_num), chapter);
        let translation_upper = translation.to_uppercase();

        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let client = crate::api::youversion::YouVersionClient::new(api_key);
            let result = match client.get_passage(version_id, &usfm) {
                Ok(passage) => {
                    let verses = crate::api::youversion::parse_passage_to_verses(
                        &passage.content,
                        book_num,
                        chapter,
                        &translation_upper,
                    );
                    if let Ok(c) = crate::api::cache::CacheDb::open() {
                        c.store_chapter(version_id, book_num, chapter, &verses);
                    }
                    Ok(verses)
                }
                Err(e) => Err(e.to_string()),
            };
            tx.send(result).ok();
        });

        self.resolver.pending = Some(PendingFetch {
            receiver: rx,
            book_idx: book_num.saturating_sub(1) as usize,
            chapter,
        });

        FetchResult::Loading
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bible::db;

    #[test]
    fn bundled_resolves_immediately() {
        let conn = db::open_db();
        let mut resolver = ChapterResolver::new();
        let mut ctx = ResolveContext {
            conn: &conn,
            resolver: &mut resolver,
            #[cfg(feature = "api")]
            cache: None,
            #[cfg(feature = "api")]
            providers: &crate::config::providers::ProvidersConfig::default(),
        };
        let result = ctx.resolve("KJV", 1, 1);
        match result {
            FetchResult::Ready(verses) => assert_eq!(verses.len(), 31),
            _ => panic!("Expected Ready for bundled translation"),
        }
    }

    #[cfg(not(feature = "api"))]
    #[test]
    fn non_bundled_without_api_returns_error() {
        let conn = db::open_db();
        let mut resolver = ChapterResolver::new();
        let mut ctx = ResolveContext {
            conn: &conn,
            resolver: &mut resolver,
        };
        let result = ctx.resolve("ASV", 1, 1);
        match result {
            FetchResult::Error(msg) => assert!(msg.contains("API feature not enabled")),
            _ => panic!("Expected Error for non-bundled without API"),
        }
    }

    #[test]
    fn poll_no_pending_returns_none() {
        let mut resolver = ChapterResolver::new();
        assert!(resolver.poll().is_none());
    }

    #[test]
    fn is_loading_initially_false() {
        let resolver = ChapterResolver::new();
        assert!(!resolver.is_loading());
    }
}
