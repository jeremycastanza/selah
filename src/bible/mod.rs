pub mod books;
pub mod db;
pub mod random;
pub mod types;

pub struct TranslationInfo {
    pub code: &'static str,
    pub name: &'static str,
    pub lang: &'static str,
    pub offline: bool,
}

pub const TRANSLATIONS: &[TranslationInfo] = &[
    // English
    TranslationInfo { code: "KJV",   name: "King James Version",          lang: "English",    offline: true  },
    TranslationInfo { code: "ASV",   name: "American Standard Version",   lang: "English",    offline: false },
    TranslationInfo { code: "BBE",   name: "Bible in Basic English",      lang: "English",    offline: false },
    TranslationInfo { code: "DARBY", name: "Darby Translation",           lang: "English",    offline: false },
    TranslationInfo { code: "YLT",   name: "Young's Literal Translation", lang: "English",    offline: false },
    TranslationInfo { code: "WEB",   name: "World English Bible",         lang: "English",    offline: false },
    TranslationInfo { code: "DRC",   name: "Douay-Rheims Catholic Bible", lang: "English",    offline: false },
    // Spanish
    TranslationInfo { code: "RVR",   name: "Reina-Valera 1960",           lang: "Español",    offline: false },
    TranslationInfo { code: "NVI",   name: "Nueva Versión Internacional", lang: "Español",    offline: false },
    // French
    TranslationInfo { code: "LSG",   name: "Louis Segond 1910",           lang: "Français",   offline: false },
    TranslationInfo { code: "NEG",   name: "Nouvelle Edition de Genève",  lang: "Français",   offline: false },
    // German
    TranslationInfo { code: "LUTH",  name: "Luther Bibel 1912",           lang: "Deutsch",    offline: false },
    // Portuguese
    TranslationInfo { code: "ARA",   name: "Almeida Revista e Atualizada", lang: "Português", offline: false },
    // Latin
    TranslationInfo { code: "VULG",  name: "Vulgata Clementina",          lang: "Latin",      offline: false },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translations_contains_kjv_offline() {
        let kjv = TRANSLATIONS.iter().find(|t| t.code == "KJV");
        assert!(kjv.is_some());
        assert!(kjv.unwrap().offline);
    }

    #[test]
    fn translations_has_non_offline_entries() {
        assert!(TRANSLATIONS.iter().any(|t| !t.offline));
    }
}
