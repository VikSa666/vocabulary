pub mod set;
pub mod word;

pub use set::WordSet;
pub use word::Word;

pub fn new_word(cathegory: &str, word: &str, translations: Vec<&str>) {
    let mut word_set = set::WordSet::new_from_file(&cathegory).unwrap(); // TODO
    word_set.register_word(&word::Word::new(word, cathegory, translations));
    word_set.write_in_file().unwrap();
}
