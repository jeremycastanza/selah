pub struct Verse {
    pub book: String,
    pub book_num: u32,
    pub chapter: u32,
    pub verse: u32,
    pub text: String,
    pub translation: String,
}

pub struct Chapter {
    pub book: String,
    pub chapter: u32,
    pub verses: Vec<Verse>,
}

pub struct SearchResult {
    pub book: String,
    pub book_num: u32,
    pub chapter: u32,
    pub verse: u32,
    pub text: String,
}
