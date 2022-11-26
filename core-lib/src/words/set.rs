use crate::error;
use crate::words::word;
use std::io::Write;

#[derive(Debug, PartialEq)]
pub struct WordSet {
    name: String,
    words: Vec<word::Word>,
}

impl WordSet {
    /// This funciton will take ownership of `self` and register a word in `self.words` by
    /// pushing it to the vector.
    pub fn register_word(&mut self, word: &word::Word) {
        self.words.push(word.clone())
    }

    pub fn get_word_by_name(&self, word: String) -> Option<&word::Word> {
        self.words.iter().find(|w| w.word == word)
    }

    pub fn get_word_refs(&self) -> Vec<&word::Word> {
        self.words.iter().collect()
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn new_from_str(contents: &str, set_name: &str) -> error::Result<Self> {
        let yamls = yaml_rust::YamlLoader::load_from_str(&contents).unwrap();
        let mut set = Self {
            name: set_name.to_string(),
            words: vec![],
        };
        for yaml in yamls {
            let new_word = word::Word::new(
                yaml["original"].as_str().unwrap(),
                set_name,
                yaml["translations"]
                    .as_vec()
                    .unwrap()
                    .into_iter()
                    .map(|c| c.as_str().unwrap())
                    .collect::<Vec<&str>>(),
            );
            set.register_word(&new_word)
        }
        Ok(set)
    }

    pub fn new_from_file(name: &str) -> error::Result<Self> {
        let file_path = format!("resources/{}.yaml", name);
        let contents = std::fs::read_to_string(file_path).map_err(|err| {
            error::Error::FileError("Should have been able to read the file".to_string())
        })?;
        let yamls = yaml_rust::YamlLoader::load_from_str(&contents).unwrap();
        let mut set = Self {
            name: name.to_string(),
            words: vec![],
        };
        for yaml in yamls {
            let new_word = word::Word::new(
                yaml["original"].as_str().unwrap(),
                name,
                yaml["translations"]
                    .as_vec()
                    .unwrap()
                    .into_iter()
                    .map(|c| c.as_str().unwrap())
                    .collect::<Vec<&str>>(),
            );
            set.register_word(&new_word)
        }
        Ok(set)
    }

    pub fn update(&mut self, other: &WordSet) {
        for word in other.words.iter() {
            if self.get_word_by_name(word.name().to_string()).is_none() {
                self.register_word(word)
            }
        }
    }

    pub fn write_in_file(&self) -> Result<(), error::Error> {
        let mut set = WordSet::new_from_file(&self.name)?;
        set.update(self);
        let buf = &std::path::PathBuf::from("./resources");
        std::fs::create_dir_all(buf).unwrap();
        let out_dir = &buf.join(format!("{}.yaml", self.name));
        let out = std::fs::File::create(out_dir).map_err(|err| {
            error::Error::FileError(format!("Could not create file on write only error. {err}"))
        })?;
        write!(
            &out,
            "{}",
            set.get_word_refs()
                .iter()
                .map(|w| w.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
        .map_err(|_| error::Error::FileError("Could not write on file".to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::word;
    use super::WordSet;

    #[test]
    fn test_write_in_file() {
        let word_set = WordSet {
            name: String::from("test"),
            words: vec![word::Word {
                word: String::from("test"),
                cathegory: String::from("test"),
                translations: vec![
                    String::from("test1"),
                    String::from("test2"),
                    String::from("test3"),
                ],
            }],
        };
        word_set.write_in_file().unwrap()
    }

    #[test]
    fn test_register_word() {
        let set = WordSet::new_from_file("test").unwrap();
        let expected = WordSet {
            name: String::from("test"),
            words: vec![word::Word {
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
