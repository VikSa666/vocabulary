use std::io::Write;

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
        let path_to_yaml = format!("resources/{}.yaml", name);
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

    pub fn write_in_file(&self) -> Result<(), std::io::Error> {
        let path = format!("resources/{}.yaml", self.name);
        let out_dir = &std::path::PathBuf::from(path.to_string());
        let out = std::fs::File::create(out_dir)?;
        write!(
            &out,
            "{}",
            self.get_word_refs()
                .iter()
                .map(|w| w.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
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

fn ask_new_word() {
    let mut s = String::new();
    print!("Cathegory: ");
    let _ = std::io::Write::flush(&mut std::io::stdout());
    std::io::stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    let cathegory = s.clone();
    s = String::new();
    print!("Word: ");
    let _ = std::io::Write::flush(&mut std::io::stdout());
    std::io::stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    let word = s.clone();
    s = String::new();
    print!("Translations: ");
    let _ = std::io::Write::flush(&mut std::io::stdout());
    std::io::stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    let translations = s
        .clone()
        .split_whitespace()
        .map(|str| str.to_string())
        .collect::<Vec<String>>();
    let mut word_set = WordSet::new(&cathegory);
    word_set.register_word(Word::new(
        word.as_str(),
        cathegory.as_str(),
        translations.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
    ));
    word_set.write_in_file().unwrap();
}

#[cfg(test)]
mod test {
    use super::{ask_new_word, Word, WordSet};

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

    #[test]
    fn test_write_word() {
        ask_new_word()
        // std::fs::File::open("resources/test.yaml").unwrap();
    }
}
