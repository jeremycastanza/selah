pub mod books;
pub mod db;
pub mod random;
pub mod resolver;
pub mod types;

pub struct TranslationInfo {
    pub code: &'static str,
    pub name: &'static str,
    pub lang: &'static str,
    pub offline: bool,
}

pub const TRANSLATIONS: &[TranslationInfo] = &[
    // Bundled
    TranslationInfo {
        code: "KJV",
        name: "King James Version",
        lang: "English",
        offline: true,
    },
    // YouVersion API — English translations (synced via Settings)
    TranslationInfo {
        code: "AMP",
        name: "Amplified Bible",
        lang: "English",
        offline: false,
    },
    TranslationInfo {
        code: "ASV",
        name: "American Standard Version",
        lang: "English",
        offline: false,
    },
    TranslationInfo {
        code: "BSB",
        name: "Berean Standard Bible",
        lang: "English",
        offline: false,
    },
    TranslationInfo {
        code: "CPDV",
        name: "Catholic Public Domain Version",
        lang: "English",
        offline: false,
    },
    TranslationInfo {
        code: "EASY",
        name: "EasyEnglish Bible 2024",
        lang: "English",
        offline: false,
    },
    TranslationInfo {
        code: "FBV",
        name: "Free Bible Version",
        lang: "English",
        offline: false,
    },
    TranslationInfo {
        code: "LSV",
        name: "Literal Standard Version",
        lang: "English",
        offline: false,
    },
    TranslationInfo {
        code: "NASB1995",
        name: "New American Standard Bible 1995",
        lang: "English",
        offline: false,
    },
    TranslationInfo {
        code: "NASB2020",
        name: "New American Standard Bible 2020",
        lang: "English",
        offline: false,
    },
    TranslationInfo {
        code: "NIV11",
        name: "New International Version",
        lang: "English",
        offline: false,
    },
    TranslationInfo {
        code: "NIrV",
        name: "New International Reader's Version",
        lang: "English",
        offline: false,
    },
    TranslationInfo {
        code: "NIVUK11",
        name: "New International Version (Anglicised)",
        lang: "English",
        offline: false,
    },
    TranslationInfo {
        code: "PEV",
        name: "Plain English Version",
        lang: "English",
        offline: false,
    },
    TranslationInfo {
        code: "TCENT",
        name: "Text-Critical English New Testament",
        lang: "English",
        offline: false,
    },
    TranslationInfo {
        code: "TOJB2011",
        name: "The Orthodox Jewish Bible",
        lang: "English",
        offline: false,
    },
    TranslationInfo {
        code: "TPT",
        name: "The Passion Translation",
        lang: "English",
        offline: false,
    },
    TranslationInfo {
        code: "WMB",
        name: "World Messianic Bible",
        lang: "English",
        offline: false,
    },
    TranslationInfo {
        code: "WMBBE",
        name: "World Messianic Bible British Edition",
        lang: "English",
        offline: false,
    },
    TranslationInfo {
        code: "engWEBUS",
        name: "World English Bible",
        lang: "English",
        offline: false,
    },
    TranslationInfo {
        code: "enggnv",
        name: "Geneva Bible",
        lang: "English",
        offline: false,
    },
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
