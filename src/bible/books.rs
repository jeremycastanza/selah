pub struct BookInfo {
    pub name: &'static str,
    pub abbreviation: &'static str,
    pub usfm: &'static str,
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
        usfm: "GEN",
        chapters: 50,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Exodus",
        abbreviation: "Exod",
        usfm: "EXO",
        chapters: 40,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Leviticus",
        abbreviation: "Lev",
        usfm: "LEV",
        chapters: 27,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Numbers",
        abbreviation: "Num",
        usfm: "NUM",
        chapters: 36,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Deuteronomy",
        abbreviation: "Deut",
        usfm: "DEU",
        chapters: 34,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Joshua",
        abbreviation: "Josh",
        usfm: "JOS",
        chapters: 24,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Judges",
        abbreviation: "Judg",
        usfm: "JDG",
        chapters: 21,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Ruth",
        abbreviation: "Ruth",
        usfm: "RUT",
        chapters: 4,
        testament: Testament::Old,
    },
    BookInfo {
        name: "1 Samuel",
        abbreviation: "1Sam",
        usfm: "1SA",
        chapters: 31,
        testament: Testament::Old,
    },
    BookInfo {
        name: "2 Samuel",
        abbreviation: "2Sam",
        usfm: "2SA",
        chapters: 24,
        testament: Testament::Old,
    },
    BookInfo {
        name: "1 Kings",
        abbreviation: "1Kgs",
        usfm: "1KI",
        chapters: 22,
        testament: Testament::Old,
    },
    BookInfo {
        name: "2 Kings",
        abbreviation: "2Kgs",
        usfm: "2KI",
        chapters: 25,
        testament: Testament::Old,
    },
    BookInfo {
        name: "1 Chronicles",
        abbreviation: "1Chr",
        usfm: "1CH",
        chapters: 29,
        testament: Testament::Old,
    },
    BookInfo {
        name: "2 Chronicles",
        abbreviation: "2Chr",
        usfm: "2CH",
        chapters: 36,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Ezra",
        abbreviation: "Ezra",
        usfm: "EZR",
        chapters: 10,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Nehemiah",
        abbreviation: "Neh",
        usfm: "NEH",
        chapters: 13,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Esther",
        abbreviation: "Esth",
        usfm: "EST",
        chapters: 10,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Job",
        abbreviation: "Job",
        usfm: "JOB",
        chapters: 42,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Psalms",
        abbreviation: "Ps",
        usfm: "PSA",
        chapters: 150,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Proverbs",
        abbreviation: "Prov",
        usfm: "PRO",
        chapters: 31,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Ecclesiastes",
        abbreviation: "Eccl",
        usfm: "ECC",
        chapters: 12,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Song of Solomon",
        abbreviation: "Song",
        usfm: "SNG",
        chapters: 8,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Isaiah",
        abbreviation: "Isa",
        usfm: "ISA",
        chapters: 66,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Jeremiah",
        abbreviation: "Jer",
        usfm: "JER",
        chapters: 52,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Lamentations",
        abbreviation: "Lam",
        usfm: "LAM",
        chapters: 5,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Ezekiel",
        abbreviation: "Ezek",
        usfm: "EZK",
        chapters: 48,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Daniel",
        abbreviation: "Dan",
        usfm: "DAN",
        chapters: 12,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Hosea",
        abbreviation: "Hos",
        usfm: "HOS",
        chapters: 14,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Joel",
        abbreviation: "Joel",
        usfm: "JOL",
        chapters: 3,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Amos",
        abbreviation: "Amos",
        usfm: "AMO",
        chapters: 9,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Obadiah",
        abbreviation: "Obad",
        usfm: "OBA",
        chapters: 1,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Jonah",
        abbreviation: "Jonah",
        usfm: "JON",
        chapters: 4,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Micah",
        abbreviation: "Mic",
        usfm: "MIC",
        chapters: 7,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Nahum",
        abbreviation: "Nah",
        usfm: "NAM",
        chapters: 3,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Habakkuk",
        abbreviation: "Hab",
        usfm: "HAB",
        chapters: 3,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Zephaniah",
        abbreviation: "Zeph",
        usfm: "ZEP",
        chapters: 3,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Haggai",
        abbreviation: "Hag",
        usfm: "HAG",
        chapters: 2,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Zechariah",
        abbreviation: "Zech",
        usfm: "ZEC",
        chapters: 14,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Malachi",
        abbreviation: "Mal",
        usfm: "MAL",
        chapters: 4,
        testament: Testament::Old,
    },
    BookInfo {
        name: "Matthew",
        abbreviation: "Matt",
        usfm: "MAT",
        chapters: 28,
        testament: Testament::New,
    },
    BookInfo {
        name: "Mark",
        abbreviation: "Mark",
        usfm: "MRK",
        chapters: 16,
        testament: Testament::New,
    },
    BookInfo {
        name: "Luke",
        abbreviation: "Luke",
        usfm: "LUK",
        chapters: 24,
        testament: Testament::New,
    },
    BookInfo {
        name: "John",
        abbreviation: "John",
        usfm: "JHN",
        chapters: 21,
        testament: Testament::New,
    },
    BookInfo {
        name: "Acts",
        abbreviation: "Acts",
        usfm: "ACT",
        chapters: 28,
        testament: Testament::New,
    },
    BookInfo {
        name: "Romans",
        abbreviation: "Rom",
        usfm: "ROM",
        chapters: 16,
        testament: Testament::New,
    },
    BookInfo {
        name: "1 Corinthians",
        abbreviation: "1Cor",
        usfm: "1CO",
        chapters: 16,
        testament: Testament::New,
    },
    BookInfo {
        name: "2 Corinthians",
        abbreviation: "2Cor",
        usfm: "2CO",
        chapters: 13,
        testament: Testament::New,
    },
    BookInfo {
        name: "Galatians",
        abbreviation: "Gal",
        usfm: "GAL",
        chapters: 6,
        testament: Testament::New,
    },
    BookInfo {
        name: "Ephesians",
        abbreviation: "Eph",
        usfm: "EPH",
        chapters: 6,
        testament: Testament::New,
    },
    BookInfo {
        name: "Philippians",
        abbreviation: "Phil",
        usfm: "PHP",
        chapters: 4,
        testament: Testament::New,
    },
    BookInfo {
        name: "Colossians",
        abbreviation: "Col",
        usfm: "COL",
        chapters: 4,
        testament: Testament::New,
    },
    BookInfo {
        name: "1 Thessalonians",
        abbreviation: "1Thess",
        usfm: "1TH",
        chapters: 5,
        testament: Testament::New,
    },
    BookInfo {
        name: "2 Thessalonians",
        abbreviation: "2Thess",
        usfm: "2TH",
        chapters: 3,
        testament: Testament::New,
    },
    BookInfo {
        name: "1 Timothy",
        abbreviation: "1Tim",
        usfm: "1TI",
        chapters: 6,
        testament: Testament::New,
    },
    BookInfo {
        name: "2 Timothy",
        abbreviation: "2Tim",
        usfm: "2TI",
        chapters: 4,
        testament: Testament::New,
    },
    BookInfo {
        name: "Titus",
        abbreviation: "Titus",
        usfm: "TIT",
        chapters: 3,
        testament: Testament::New,
    },
    BookInfo {
        name: "Philemon",
        abbreviation: "Phlm",
        usfm: "PHM",
        chapters: 1,
        testament: Testament::New,
    },
    BookInfo {
        name: "Hebrews",
        abbreviation: "Heb",
        usfm: "HEB",
        chapters: 13,
        testament: Testament::New,
    },
    BookInfo {
        name: "James",
        abbreviation: "Jas",
        usfm: "JAS",
        chapters: 5,
        testament: Testament::New,
    },
    BookInfo {
        name: "1 Peter",
        abbreviation: "1Pet",
        usfm: "1PE",
        chapters: 5,
        testament: Testament::New,
    },
    BookInfo {
        name: "2 Peter",
        abbreviation: "2Pet",
        usfm: "2PE",
        chapters: 3,
        testament: Testament::New,
    },
    BookInfo {
        name: "1 John",
        abbreviation: "1John",
        usfm: "1JN",
        chapters: 5,
        testament: Testament::New,
    },
    BookInfo {
        name: "2 John",
        abbreviation: "2John",
        usfm: "2JN",
        chapters: 1,
        testament: Testament::New,
    },
    BookInfo {
        name: "3 John",
        abbreviation: "3John",
        usfm: "3JN",
        chapters: 1,
        testament: Testament::New,
    },
    BookInfo {
        name: "Jude",
        abbreviation: "Jude",
        usfm: "JUD",
        chapters: 1,
        testament: Testament::New,
    },
    BookInfo {
        name: "Revelation",
        abbreviation: "Rev",
        usfm: "REV",
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

pub fn book_usfm(book_num: u32) -> &'static str {
    BOOKS
        .get((book_num - 1) as usize)
        .map(|b| b.usfm)
        .unwrap_or("UNK")
}

pub fn book_num_from_usfm(usfm: &str) -> Option<u32> {
    BOOKS
        .iter()
        .position(|b| b.usfm == usfm)
        .map(|i| (i + 1) as u32)
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

    #[test]
    fn book_usfm_genesis() {
        assert_eq!(book_usfm(1), "GEN");
    }

    #[test]
    fn book_usfm_john() {
        assert_eq!(book_usfm(43), "JHN");
    }

    #[test]
    fn book_usfm_revelation() {
        assert_eq!(book_usfm(66), "REV");
    }

    #[test]
    fn book_num_from_usfm_genesis() {
        assert_eq!(book_num_from_usfm("GEN"), Some(1));
    }

    #[test]
    fn book_num_from_usfm_john() {
        assert_eq!(book_num_from_usfm("JHN"), Some(43));
    }

    #[test]
    fn book_num_from_usfm_unknown() {
        assert_eq!(book_num_from_usfm("XYZ"), None);
    }

    #[test]
    fn usfm_round_trip_all_books() {
        for n in 1..=66 {
            assert_eq!(book_num_from_usfm(book_usfm(n)), Some(n));
        }
    }
}
