pub struct Word {
    word: String,
    cathegory: String,
    translations: Vec<String>,
}

impl Word {
    pub fn new(word: &str, cathegory: &str, translations: &[&str]) -> Self {
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
}

pub enum Error {
    NotImplemented,
}

pub struct WordSet {
    name: String,
    words: Vec<Word>,
}

impl WordSet {
    /// This funciton will take ownership of `self` and register a word in `self.words` by
    /// pushing it to the vector.
    pub fn register_word(mut self, word: Word) {
        self.words.push(word)
    }

    pub fn get_word_by_name(&self, word: String) -> Result<&Word, Error> {
        self.words
            .iter()
            .find(|w| w.word == word)
            .ok_or(Error::NotImplemented)
    }
}
