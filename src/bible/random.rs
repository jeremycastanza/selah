use rusqlite::Connection;

use super::db;
use super::types::Verse;

pub fn random_verse(conn: &Connection, translation: &str) -> Option<Verse> {
    db::get_random_verse(conn, translation)
}
