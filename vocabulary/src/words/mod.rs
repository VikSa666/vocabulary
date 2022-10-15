#[derive(Debug, PartialEq)]
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
}

pub enum Error {
    NotImplemented,
}

#[derive(Debug, PartialEq)]
pub struct WordSet {
    name: String,
    words: Vec<Word>,
}

impl WordSet {
    /// This funciton will take ownership of `self` and register a word in `self.words` by
    /// pushing it to the vector.
    pub fn register_word(&mut self, word: Word) {
        self.words.push(word)
    }

    pub fn get_word_by_name(&self, word: String) -> Result<&Word, Error> {
        self.words
            .iter()
            .find(|w| w.word == word)
            .ok_or(Error::NotImplemented)
    }

    pub fn get_word_refs(&self) -> Vec<&Word> {
        self.words.iter().collect()
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn new(name: &str) -> Self {
        let path_to_yaml = format!("../../vocabulary/resources/{}.yaml", name);
        let source = std::fs::read_to_string(path_to_yaml).unwrap();
        let yamls = yaml_rust::YamlLoader::load_from_str(source.as_str()).unwrap();
        let mut set = Self {
            name: name.to_string(),
            words: vec![],
        };
        for yaml in yamls {
            let new_word = Word::new(
                yaml["original"].as_str().unwrap(),
                name,
                yaml["translations"]
                    .as_vec()
                    .unwrap()
                    .into_iter()
                    .map(|c| c.as_str().unwrap())
                    .collect::<Vec<&str>>(),
            );
            set.register_word(new_word)
        }
        set
    }
}

#[cfg(test)]
mod test {
    use super::{Word, WordSet};

    #[test]
    fn test_register_word() {
        let set = WordSet::new("test");
        let expected = WordSet {
            name: String::from("test"),
            words: vec![Word {
                word: String::from("test"),
                cathegory: String::from("test"),
                translations: vec![
                    String::from("test1"),
                    String::from("test2"),
                    String::from("test3"),
                ],
            }],
        };
        pretty_assertions::assert_eq!(set, expected)
    }
}
