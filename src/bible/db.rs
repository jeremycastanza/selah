use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

use directories::ProjectDirs;
use rusqlite::Connection;

use super::books::book_name;
use super::types::{SearchResult, Verse};

const KJV_DB: &[u8] = include_bytes!("../../data/kjv.sqlite");

static DB_COUNTER: AtomicU32 = AtomicU32::new(0);

pub fn open_db() -> Connection {
    let id = DB_COUNTER.fetch_add(1, Ordering::Relaxed);
    let tmp_path =
        std::env::temp_dir().join(format!("selah-kjv-{}-{id}.sqlite", std::process::id()));
    fs::write(&tmp_path, KJV_DB).expect("Failed to write KJV database to temp dir");
    let conn = Connection::open(&tmp_path).expect("Failed to open KJV database");
    conn.execute_batch("PRAGMA journal_mode=OFF; PRAGMA synchronous=OFF;")
        .expect("Failed to set pragmas");
    build_fts(&conn, "kjv");
    conn
}

fn cache_db_path() -> Option<PathBuf> {
    let dirs = ProjectDirs::from("", "", "selah")?;
    let path = dirs.data_dir().join("cache.sqlite");
    if path.exists() { Some(path) } else { None }
}

fn open_cache_db() -> Option<Connection> {
    let path = cache_db_path()?;
    Connection::open_with_flags(&path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY).ok()
}

fn build_fts(conn: &Connection, translation: &str) {
    let table = verse_table(translation);
    let fts = format!("{}_fts", table);
    conn.execute_batch(&format!(
        "CREATE VIRTUAL TABLE IF NOT EXISTS {fts} USING fts5(t, content='{table}', content_rowid='rowid');\
         INSERT INTO {fts}({fts}) VALUES('rebuild');",
    ))
    .expect("Failed to build FTS index");
}

fn verse_table(translation: &str) -> String {
    format!("t_{}", translation.to_lowercase())
}

pub fn get_chapter(
    conn: &Connection,
    translation: &str,
    book_num: u32,
    chapter: u32,
) -> Vec<Verse> {
    let table = verse_table(translation);
    let sql = format!(
        "SELECT b, c, v, t FROM {} WHERE b = ?1 AND c = ?2 ORDER BY v",
        table
    );
    let mut stmt = match conn.prepare(&sql) {
        Ok(s) => s,
        Err(_) => return get_chapter_from_cache(translation, book_num, chapter),
    };
    match stmt.query_map(rusqlite::params![book_num, chapter], |row| {
        let b: u32 = row.get(0)?;
        Ok(Verse {
            book: book_name(b).to_string(),
            book_num: b,
            chapter: row.get(1)?,
            verse: row.get(2)?,
            text: row.get(3)?,
            translation: translation.to_uppercase(),
        })
    }) {
        Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
        Err(_) => get_chapter_from_cache(translation, book_num, chapter),
    }
}

fn get_chapter_from_cache(translation: &str, book_num: u32, chapter: u32) -> Vec<Verse> {
    let Some(cache) = open_cache_db() else {
        return vec![];
    };
    let sql = "SELECT book_num, chapter, verse, text FROM cached_verses \
               WHERE translation = ?1 AND book_num = ?2 AND chapter = ?3 ORDER BY verse";
    let mut stmt = match cache.prepare(sql) {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    stmt.query_map(rusqlite::params![translation.to_uppercase(), book_num, chapter], |row| {
        let b: u32 = row.get(0)?;
        Ok(Verse {
            book: book_name(b).to_string(),
            book_num: b,
            chapter: row.get(1)?,
            verse: row.get(2)?,
            text: row.get(3)?,
            translation: translation.to_uppercase(),
        })
    })
    .map(|rows| rows.filter_map(|r| r.ok()).collect())
    .unwrap_or_default()
}

pub fn get_verse(
    conn: &Connection,
    translation: &str,
    book_num: u32,
    chapter: u32,
    verse: u32,
) -> Option<Verse> {
    let table = verse_table(translation);
    let sql = format!(
        "SELECT b, c, v, t FROM {} WHERE b = ?1 AND c = ?2 AND v = ?3",
        table
    );
    match conn.prepare(&sql) {
        Ok(mut stmt) => stmt
            .query_row(rusqlite::params![book_num, chapter, verse], |row| {
                let b: u32 = row.get(0)?;
                Ok(Verse {
                    book: book_name(b).to_string(),
                    book_num: b,
                    chapter: row.get(1)?,
                    verse: row.get(2)?,
                    text: row.get(3)?,
                    translation: translation.to_uppercase(),
                })
            })
            .ok(),
        Err(_) => get_verse_from_cache(translation, book_num, chapter, verse),
    }
}

fn get_verse_from_cache(
    translation: &str,
    book_num: u32,
    chapter: u32,
    verse: u32,
) -> Option<Verse> {
    let cache = open_cache_db()?;
    let sql = "SELECT book_num, chapter, verse, text FROM cached_verses \
               WHERE translation = ?1 AND book_num = ?2 AND chapter = ?3 AND verse = ?4";
    let mut stmt = cache.prepare(sql).ok()?;
    stmt.query_row(
        rusqlite::params![translation.to_uppercase(), book_num, chapter, verse],
        |row| {
            let b: u32 = row.get(0)?;
            Ok(Verse {
                book: book_name(b).to_string(),
                book_num: b,
                chapter: row.get(1)?,
                verse: row.get(2)?,
                text: row.get(3)?,
                translation: translation.to_uppercase(),
            })
        },
    )
    .ok()
}

pub fn search(conn: &Connection, query: &str, translation: &str) -> Vec<SearchResult> {
    if query.len() < 3 {
        return vec![];
    }
    let table = verse_table(translation);
    let fts_table = format!("{}_fts", table);
    let sql = format!(
        "SELECT {t}.b, {t}.c, {t}.v, {t}.t \
         FROM {fts} JOIN {t} ON {t}.rowid = {fts}.rowid \
         WHERE {fts} MATCH ?1 \
         ORDER BY {fts}.rank LIMIT 50",
        t = table,
        fts = fts_table,
    );
    let mut stmt = match conn.prepare(&sql) {
        Ok(s) => s,
        Err(_) => return search_cache(query, translation),
    };
    stmt.query_map(rusqlite::params![query], |row| {
        let b: u32 = row.get(0)?;
        Ok(SearchResult {
            book: book_name(b).to_string(),
            book_num: b,
            chapter: row.get(1)?,
            verse: row.get(2)?,
            text: row.get(3)?,
        })
    })
    .map(|rows| rows.filter_map(|r| r.ok()).collect())
    .unwrap_or_default()
}

fn search_cache(query: &str, translation: &str) -> Vec<SearchResult> {
    let Some(cache) = open_cache_db() else {
        return vec![];
    };
    let pattern = format!("%{query}%");
    let sql = "SELECT book_num, chapter, verse, text FROM cached_verses \
               WHERE translation = ?1 AND text LIKE ?2 LIMIT 50";
    let mut stmt = match cache.prepare(sql) {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    stmt.query_map(rusqlite::params![translation.to_uppercase(), pattern], |row| {
        let b: u32 = row.get(0)?;
        Ok(SearchResult {
            book: book_name(b).to_string(),
            book_num: b,
            chapter: row.get(1)?,
            verse: row.get(2)?,
            text: row.get(3)?,
        })
    })
    .map(|rows| rows.filter_map(|r| r.ok()).collect())
    .unwrap_or_default()
}

pub fn get_random_verse(conn: &Connection, translation: &str) -> Option<Verse> {
    let table = verse_table(translation);
    let sql = format!("SELECT b, c, v, t FROM {} ORDER BY RANDOM() LIMIT 1", table);
    match conn.prepare(&sql) {
        Ok(mut stmt) => stmt
            .query_row([], |row| {
                let b: u32 = row.get(0)?;
                Ok(Verse {
                    book: book_name(b).to_string(),
                    book_num: b,
                    chapter: row.get(1)?,
                    verse: row.get(2)?,
                    text: row.get(3)?,
                    translation: translation.to_uppercase(),
                })
            })
            .ok(),
        Err(_) => get_random_verse_from_cache(translation),
    }
}

fn get_random_verse_from_cache(translation: &str) -> Option<Verse> {
    let cache = open_cache_db()?;
    let sql = "SELECT book_num, chapter, verse, text FROM cached_verses \
               WHERE translation = ?1 ORDER BY RANDOM() LIMIT 1";
    let mut stmt = cache.prepare(sql).ok()?;
    stmt.query_row(rusqlite::params![translation.to_uppercase()], |row| {
        let b: u32 = row.get(0)?;
        Ok(Verse {
            book: book_name(b).to_string(),
            book_num: b,
            chapter: row.get(1)?,
            verse: row.get(2)?,
            text: row.get(3)?,
            translation: translation.to_uppercase(),
        })
    })
    .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_db() -> Connection {
        open_db()
    }

    #[test]
    fn genesis_1_has_31_verses() {
        let conn = test_db();
        let verses = get_chapter(&conn, "kjv", 1, 1);
        assert_eq!(verses.len(), 31);
    }

    #[test]
    fn john_3_16() {
        let conn = test_db();
        let verse = get_verse(&conn, "kjv", 43, 3, 16).expect("John 3:16 should exist");
        assert_eq!(verse.book, "John");
        assert_eq!(verse.chapter, 3);
        assert_eq!(verse.verse, 16);
        assert!(verse.text.contains("God so loved"));
    }

    #[test]
    fn search_love_returns_results() {
        let conn = test_db();
        let results = search(&conn, "love", "kjv");
        assert!(!results.is_empty());
    }

    #[test]
    fn search_faith_returns_results() {
        let conn = test_db();
        let results = search(&conn, "faith", "kjv");
        assert!(!results.is_empty());
    }

    #[test]
    fn search_short_query_returns_empty() {
        let conn = test_db();
        let results = search(&conn, "ab", "kjv");
        assert!(results.is_empty());
    }

    #[test]
    fn search_nonexistent_returns_empty() {
        let conn = test_db();
        let results = search(&conn, "xyznonexistent", "kjv");
        assert!(results.is_empty());
    }

    #[test]
    fn random_verse_returns_valid() {
        let conn = test_db();
        let verse = get_random_verse(&conn, "kjv").expect("Should return a random verse");
        assert!(verse.book_num >= 1 && verse.book_num <= 66);
        assert!(!verse.text.is_empty());
    }

    #[test]
    fn get_chapter_missing_translation_returns_empty() {
        let conn = test_db();
        let verses = get_chapter(&conn, "nasb", 1, 1);
        assert!(verses.is_empty());
    }
}
