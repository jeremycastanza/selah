pub struct BookInfo {
    pub name: &'static str,
    pub abbreviation: &'static str,
    pub chapters: u32,
    pub testament: Testament,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Testament {
    Old,
    New,
}

pub const BOOKS: &[BookInfo] = &[
    BookInfo {
        name: "Genesis",
        abbreviation: "Gen",
        chapters: 50,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Exodus",
        abbreviation: "Exod",
        chapters: 40,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Leviticus",
        abbreviation: "Lev",
        chapters: 27,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Numbers",
        abbreviation: "Num",
        chapters: 36,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Deuteronomy",
        abbreviation: "Deut",
        chapters: 34,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Joshua",
        abbreviation: "Josh",
        chapters: 24,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Judges",
        abbreviation: "Judg",
        chapters: 21,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Ruth",
        abbreviation: "Ruth",
        chapters: 4,
        testament: Testament::Old,
    },
    BookInfo {
        name: "1 Samuel",
        abbreviation: "1Sam",
        chapters: 31,
        testament: Testament::Old,
    },
    BookInfo {
        name: "2 Samuel",
        abbreviation: "2Sam",
        chapters: 24,
        testament: Testament::Old,
    },
    BookInfo {
        name: "1 Kings",
        abbreviation: "1Kgs",
        chapters: 22,
        testament: Testament::Old,
    },
    BookInfo {
        name: "2 Kings",
        abbreviation: "2Kgs",
        chapters: 25,
        testament: Testament::Old,
    },
    BookInfo {
        name: "1 Chronicles",
        abbreviation: "1Chr",
        chapters: 29,
        testament: Testament::Old,
    },
    BookInfo {
        name: "2 Chronicles",
        abbreviation: "2Chr",
        chapters: 36,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Ezra",
        abbreviation: "Ezra",
        chapters: 10,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Nehemiah",
        abbreviation: "Neh",
        chapters: 13,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Esther",
        abbreviation: "Esth",
        chapters: 10,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Job",
        abbreviation: "Job",
        chapters: 42,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Psalms",
        abbreviation: "Ps",
        chapters: 150,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Proverbs",
        abbreviation: "Prov",
        chapters: 31,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Ecclesiastes",
        abbreviation: "Eccl",
        chapters: 12,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Song of Solomon",
        abbreviation: "Song",
        chapters: 8,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Isaiah",
        abbreviation: "Isa",
        chapters: 66,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Jeremiah",
        abbreviation: "Jer",
        chapters: 52,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Lamentations",
        abbreviation: "Lam",
        chapters: 5,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Ezekiel",
        abbreviation: "Ezek",
        chapters: 48,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Daniel",
        abbreviation: "Dan",
        chapters: 12,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Hosea",
        abbreviation: "Hos",
        chapters: 14,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Joel",
        abbreviation: "Joel",
        chapters: 3,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Amos",
        abbreviation: "Amos",
        chapters: 9,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Obadiah",
        abbreviation: "Obad",
        chapters: 1,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Jonah",
        abbreviation: "Jonah",
        chapters: 4,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Micah",
        abbreviation: "Mic",
        chapters: 7,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Nahum",
        abbreviation: "Nah",
        chapters: 3,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Habakkuk",
        abbreviation: "Hab",
        chapters: 3,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Zephaniah",
        abbreviation: "Zeph",
        chapters: 3,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Haggai",
        abbreviation: "Hag",
        chapters: 2,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Zechariah",
        abbreviation: "Zech",
        chapters: 14,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Malachi",
        abbreviation: "Mal",
        chapters: 4,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Matthew",
        abbreviation: "Matt",
        chapters: 28,
        testament: Testament::New,
    },
    BookInfo {
        name: "Mark",
        abbreviation: "Mark",
        chapters: 16,
        testament: Testament::New,
    },
    BookInfo {
        name: "Luke",
        abbreviation: "Luke",
        chapters: 24,
        testament: Testament::New,
    },
    BookInfo {
        name: "John",
        abbreviation: "John",
        chapters: 21,
        testament: Testament::New,
    },
    BookInfo {
        name: "Acts",
        abbreviation: "Acts",
        chapters: 28,
        testament: Testament::New,
    },
    BookInfo {
        name: "Romans",
        abbreviation: "Rom",
        chapters: 16,
        testament: Testament::New,
    },
    BookInfo {
        name: "1 Corinthians",
        abbreviation: "1Cor",
        chapters: 16,
        testament: Testament::New,
    },
    BookInfo {
        name: "2 Corinthians",
        abbreviation: "2Cor",
        chapters: 13,
        testament: Testament::New,
    },
    BookInfo {
        name: "Galatians",
        abbreviation: "Gal",
        chapters: 6,
        testament: Testament::New,
    },
    BookInfo {
        name: "Ephesians",
        abbreviation: "Eph",
        chapters: 6,
        testament: Testament::New,
    },
    BookInfo {
        name: "Philippians",
        abbreviation: "Phil",
        chapters: 4,
        testament: Testament::New,
    },
    BookInfo {
        name: "Colossians",
        abbreviation: "Col",
        chapters: 4,
        testament: Testament::New,
    },
    BookInfo {
        name: "1 Thessalonians",
        abbreviation: "1Thess",
        chapters: 5,
        testament: Testament::New,
    },
    BookInfo {
        name: "2 Thessalonians",
        abbreviation: "2Thess",
        chapters: 3,
        testament: Testament::New,
    },
    BookInfo {
        name: "1 Timothy",
        abbreviation: "1Tim",
        chapters: 6,
        testament: Testament::New,
    },
    BookInfo {
        name: "2 Timothy",
        abbreviation: "2Tim",
        chapters: 4,
        testament: Testament::New,
    },
    BookInfo {
        name: "Titus",
        abbreviation: "Titus",
        chapters: 3,
        testament: Testament::New,
    },
    BookInfo {
        name: "Philemon",
        abbreviation: "Phlm",
        chapters: 1,
        testament: Testament::New,
    },
    BookInfo {
        name: "Hebrews",
        abbreviation: "Heb",
        chapters: 13,
        testament: Testament::New,
    },
    BookInfo {
        name: "James",
        abbreviation: "Jas",
        chapters: 5,
        testament: Testament::New,
    },
    BookInfo {
        name: "1 Peter",
        abbreviation: "1Pet",
        chapters: 5,
        testament: Testament::New,
    },
    BookInfo {
        name: "2 Peter",
        abbreviation: "2Pet",
        chapters: 3,
        testament: Testament::New,
    },
    BookInfo {
        name: "1 John",
        abbreviation: "1John",
        chapters: 5,
        testament: Testament::New,
    },
    BookInfo {
        name: "2 John",
        abbreviation: "2John",
        chapters: 1,
        testament: Testament::New,
    },
    BookInfo {
        name: "3 John",
        abbreviation: "3John",
        chapters: 1,
        testament: Testament::New,
    },
    BookInfo {
        name: "Jude",
        abbreviation: "Jude",
        chapters: 1,
        testament: Testament::New,
    },
    BookInfo {
        name: "Revelation",
        abbreviation: "Rev",
        chapters: 22,
        testament: Testament::New,
    },
];

pub fn book_name(book_num: u32) -> &'static str {
    BOOKS
        .get((book_num - 1) as usize)
        .map(|b| b.name)
        .unwrap_or("Unknown")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn book_count_is_66() {
        assert_eq!(BOOKS.len(), 66);
    }

    #[test]
    fn first_book_is_genesis() {
        assert_eq!(BOOKS[0].name, "Genesis");
    }

    #[test]
    fn last_book_is_revelation() {
        assert_eq!(BOOKS[65].name, "Revelation");
    }

    #[test]
    fn book_name_lookup() {
        assert_eq!(book_name(1), "Genesis");
        assert_eq!(book_name(66), "Revelation");
        assert_eq!(book_name(43), "John");
    }
}
