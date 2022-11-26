pub mod action;

use crate::{error, Word, WordSet};

pub struct Session {
    words_buffer: Vec<WordSet>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            words_buffer: Vec::new(),
        }
    }

    pub fn execute_action(&mut self, action: action::Action) -> error::Result<()> {
        match action {
            action::Action::AddWord {
                word,
                translations,
                set,
            } => {
                let mut word_set = WordSet::new_from_file(&set).unwrap(); // TODO
                let translations = translations.split(", ").collect::<String>();
                let translations = translations.split_ascii_whitespace().collect::<Vec<&str>>();
                word_set.register_word(&Word::new(&word, &set, translations));

                self.words_buffer.push(word_set);
                Ok(())
            }
            action::Action::UpdateWordFiles => {
                for word_set in self.words_buffer.iter() {
                    word_set
                        .write_in_file()
                        .map_err(|err| error::Error::FileError(err.to_string()))?;
                }
                Ok(())
            }
            action::Action::DeleteWord { word, set } => todo!(),
        }
    }
}
