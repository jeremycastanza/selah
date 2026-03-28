pub struct BrowserState {
    pub translation: String,
}

impl BrowserState {
    pub fn new(translation: String) -> Self {
        Self { translation }
    }
}
