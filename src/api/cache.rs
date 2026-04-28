use rusqlite::{Connection, params};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::bible::books::book_name;
use crate::bible::types::Verse;
use super::youversion::YvVersion;

const CACHE_TTL_DAYS: i64 = 90;

pub struct CachedVersion {
    pub version_id: u32,
    pub abbreviation: String,
    pub title: String,
    pub language_tag: String,
    pub copyright: Option<String>,
    pub books: Vec<String>,
}

pub struct CacheDb {
    conn: Connection,
}

impl CacheDb {
    pub fn open() -> Result<Self, rusqlite::Error> {
        let proj = directories::ProjectDirs::from("", "", "selah")
            .ok_or(rusqlite::Error::InvalidParameterName("no data dir".into()))?;
        let data_dir = proj.data_dir();
        std::fs::create_dir_all(data_dir).map_err(|e| {
            rusqlite::Error::InvalidParameterName(format!("create dir: {e}"))
        })?;
        let db_path = data_dir.join("cache.sqlite");
        let conn = Connection::open(db_path)?;
        Self::init_tables(&conn)?;
        Ok(Self { conn })
    }

    #[cfg(test)]
    fn open_in_memory() -> Self {
        let conn = Connection::open_in_memory().expect("in-memory db");
        Self::init_tables(&conn).expect("init tables");
        Self { conn }
    }

    fn init_tables(conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS cached_verses (
                version_id INTEGER NOT NULL,
                book_num INTEGER NOT NULL,
                chapter INTEGER NOT NULL,
                verse INTEGER NOT NULL,
                text TEXT NOT NULL,
                translation TEXT NOT NULL,
                fetched_at INTEGER NOT NULL,
                PRIMARY KEY (version_id, book_num, chapter, verse)
            );
            CREATE TABLE IF NOT EXISTS cached_versions (
                version_id INTEGER PRIMARY KEY,
                abbreviation TEXT NOT NULL,
                title TEXT NOT NULL,
                language_tag TEXT NOT NULL,
                copyright TEXT,
                books_json TEXT NOT NULL,
                fetched_at INTEGER NOT NULL
            );",
        )
    }

    pub fn get_chapter(&self, version_id: u32, book_num: u32, chapter: u32) -> Option<Vec<Verse>> {
        let mut stmt = self.conn.prepare(
            "SELECT verse, text, translation, fetched_at FROM cached_verses
             WHERE version_id = ?1 AND book_num = ?2 AND chapter = ?3
             ORDER BY verse",
        ).ok()?;

        let rows: Vec<(u32, String, String, i64)> = stmt
            .query_map(params![version_id, book_num, chapter], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })
            .ok()?
            .filter_map(|r| r.ok())
            .collect();

        if rows.is_empty() {
            return None;
        }

        if !Self::is_fresh(rows[0].3) {
            return None;
        }

        let book = book_name(book_num).to_string();
        Some(
            rows.into_iter()
                .map(|(verse, text, translation, _)| Verse {
                    book: book.clone(),
                    book_num,
                    chapter,
                    verse,
                    text,
                    translation,
                })
                .collect(),
        )
    }

    pub fn store_chapter(&self, version_id: u32, book_num: u32, chapter: u32, verses: &[Verse]) {
        let now = now_unix();
        for v in verses {
            let _ = self.conn.execute(
                "INSERT OR REPLACE INTO cached_verses
                 (version_id, book_num, chapter, verse, text, translation, fetched_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![version_id, book_num, chapter, v.verse, v.text, v.translation, now],
            );
        }
    }

    pub fn get_versions(&self) -> Vec<CachedVersion> {
        let mut stmt = match self.conn.prepare(
            "SELECT version_id, abbreviation, title, language_tag, copyright, books_json, fetched_at
             FROM cached_versions",
        ) {
            Ok(s) => s,
            Err(_) => return vec![],
        };

        stmt.query_map([], |row| {
            let fetched_at: i64 = row.get(6)?;
            let books_json: String = row.get(5)?;
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                books_json,
                fetched_at,
            ))
        })
        .map(|rows| {
            rows.filter_map(|r| r.ok())
                .filter(|(_, _, _, _, _, _, fetched_at): &(u32, String, String, String, Option<String>, String, i64)| {
                    Self::is_fresh(*fetched_at)
                })
                .map(|(version_id, abbreviation, title, language_tag, copyright, books_json, _)| {
                    let books: Vec<String> =
                        serde_json::from_str(&books_json).unwrap_or_default();
                    CachedVersion {
                        version_id,
                        abbreviation,
                        title,
                        language_tag,
                        copyright,
                        books,
                    }
                })
                .collect()
        })
        .unwrap_or_default()
    }

    pub fn store_versions(&self, versions: &[YvVersion]) {
        let now = now_unix();
        for v in versions {
            let books_json = serde_json::to_string(&v.books).unwrap_or_default();
            let _ = self.conn.execute(
                "INSERT OR REPLACE INTO cached_versions
                 (version_id, abbreviation, title, language_tag, copyright, books_json, fetched_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    v.id,
                    v.abbreviation,
                    v.localized_title,
                    v.language_tag,
                    v.copyright,
                    books_json,
                    now,
                ],
            );
        }
    }

    pub fn is_fresh(fetched_at: i64) -> bool {
        let now = now_unix();
        let ttl_secs = CACHE_TTL_DAYS * 24 * 60 * 60;
        now - fetched_at < ttl_secs
    }

    pub fn evict_expired(&self) {
        let cutoff = now_unix() - CACHE_TTL_DAYS * 24 * 60 * 60;
        let _ = self.conn.execute(
            "DELETE FROM cached_verses WHERE fetched_at < ?1",
            params![cutoff],
        );
        let _ = self.conn.execute(
            "DELETE FROM cached_versions WHERE fetched_at < ?1",
            params![cutoff],
        );
    }
}

fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_verses(book_num: u32, chapter: u32, count: u32) -> Vec<Verse> {
        (1..=count)
            .map(|v| Verse {
                book: book_name(book_num).to_string(),
                book_num,
                chapter,
                verse: v,
                text: format!("Verse {v} text"),
                translation: "TEST".to_string(),
            })
            .collect()
    }

    #[test]
    fn test_store_and_retrieve_chapter() {
        let cache = CacheDb::open_in_memory();
        let verses = make_verses(1, 1, 3);
        cache.store_chapter(1, 1, 1, &verses);

        let result = cache.get_chapter(1, 1, 1).expect("should return cached chapter");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].verse, 1);
        assert_eq!(result[0].text, "Verse 1 text");
        assert_eq!(result[0].book, "Genesis");
        assert_eq!(result[2].verse, 3);
    }

    #[test]
    fn test_expired_chapter_returns_none() {
        let cache = CacheDb::open_in_memory();
        let old_ts = now_unix() - (91 * 24 * 60 * 60);
        cache.conn.execute(
            "INSERT INTO cached_verses (version_id, book_num, chapter, verse, text, translation, fetched_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![1, 1, 1, 1, "Old verse", "TEST", old_ts],
        ).unwrap();

        assert!(cache.get_chapter(1, 1, 1).is_none());
    }

    #[test]
    fn test_store_and_retrieve_versions() {
        let cache = CacheDb::open_in_memory();
        let versions = vec![
            YvVersion {
                id: 1,
                abbreviation: "KJV".to_string(),
                localized_title: "King James Version".to_string(),
                language_tag: "en".to_string(),
                copyright: Some("Public Domain".to_string()),
                books: vec!["GEN".to_string(), "EXO".to_string()],
            },
        ];
        cache.store_versions(&versions);

        let result = cache.get_versions();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].version_id, 1);
        assert_eq!(result[0].abbreviation, "KJV");
        assert_eq!(result[0].title, "King James Version");
        assert_eq!(result[0].language_tag, "en");
        assert_eq!(result[0].copyright, Some("Public Domain".to_string()));
        assert_eq!(result[0].books, vec!["GEN", "EXO"]);
    }

    #[test]
    fn test_evict_expired() {
        let cache = CacheDb::open_in_memory();
        let old_ts = now_unix() - (91 * 24 * 60 * 60);

        // Insert old verse
        cache.conn.execute(
            "INSERT INTO cached_verses (version_id, book_num, chapter, verse, text, translation, fetched_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![1, 1, 1, 1, "Old verse", "TEST", old_ts],
        ).unwrap();

        // Insert fresh verse
        let fresh = make_verses(1, 2, 1);
        cache.store_chapter(1, 1, 2, &fresh);

        // Insert old version
        cache.conn.execute(
            "INSERT INTO cached_versions (version_id, abbreviation, title, language_tag, copyright, books_json, fetched_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![99, "OLD", "Old Version", "en", None::<String>, "[]", old_ts],
        ).unwrap();

        // Insert fresh version
        cache.store_versions(&[YvVersion {
            id: 100,
            abbreviation: "NEW".to_string(),
            localized_title: "New Version".to_string(),
            language_tag: "en".to_string(),
            copyright: None,
            books: vec![],
        }]);

        cache.evict_expired();

        // Old entries removed
        let verse_count: i64 = cache.conn.query_row(
            "SELECT COUNT(*) FROM cached_verses WHERE version_id = 1 AND chapter = 1",
            [], |row| row.get(0),
        ).unwrap();
        assert_eq!(verse_count, 0);

        // Fresh entries remain
        let fresh_count: i64 = cache.conn.query_row(
            "SELECT COUNT(*) FROM cached_verses WHERE version_id = 1 AND chapter = 2",
            [], |row| row.get(0),
        ).unwrap();
        assert_eq!(fresh_count, 1);

        let version_count: i64 = cache.conn.query_row(
            "SELECT COUNT(*) FROM cached_versions WHERE version_id = 99",
            [], |row| row.get(0),
        ).unwrap();
        assert_eq!(version_count, 0);

        let fresh_version_count: i64 = cache.conn.query_row(
            "SELECT COUNT(*) FROM cached_versions WHERE version_id = 100",
            [], |row| row.get(0),
        ).unwrap();
        assert_eq!(fresh_version_count, 1);
    }

    #[test]
    fn test_is_fresh() {
        let fresh_ts = now_unix() - (89 * 24 * 60 * 60);
        assert!(CacheDb::is_fresh(fresh_ts));

        let stale_ts = now_unix() - (91 * 24 * 60 * 60);
        assert!(!CacheDb::is_fresh(stale_ts));
    }
}
