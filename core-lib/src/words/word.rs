#[derive(Debug, PartialEq, Clone)]
pub struct Word {
    pub word: String,
    pub cathegory: String,
    pub translations: Vec<String>,
}

impl Word {
    pub fn new(word: &str, cathegory: &str, translations: Vec<&str>) -> Self {
        Self {
            word: word.to_string(),
            cathegory: cathegory.to_string(),
            translations: translations
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .to_owned(),
        }
    }
    pub fn name(&self) -> &str {
        &self.word
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n---\noriginal: {}\ntranslations: [{}]",
            self.word,
            self.translations.join(", ")
        )
    }
}
