use std::io::{stdin, stdout, Write};

use crate::words::{Word, WordSet};

use rand::{self, seq::SliceRandom, thread_rng};
pub struct ExamOptions {} // Todo

impl ExamOptions {
    fn options_are_valid(&self) -> bool {
        true // todo
    }
}

pub fn start_exam(options: ExamOptions) {
    let set = WordSet::new("transports");
    let mut words = set.get_word_refs();
    words.shuffle(&mut thread_rng());
    let mut s = String::new();
    for word in words.iter() {
        match ask_word_by_terminal(word) {
            AskWordResult::Correct => println!("Correct!!"),
            AskWordResult::Wrong => println!("That is wrong"),
        }
    }
}

fn ask_word_by_terminal(word: &Word) -> AskWordResult {
    let mut s = String::new();
    print!("{}: ", word.word);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    AskWordResult::correct_word(word, s.as_str())
}

pub enum AskWordResult {
    Correct,
    Wrong,
}

impl AskWordResult {
    pub fn correct_word(word: &Word, answer: &str) -> Self {
        if word.translations.contains(&answer.to_string()) {
            return Self::Correct;
        } else {
            return Self::Wrong;
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::{stdin, stdout, Write};

    use crate::{exams::ExamOptions, start_exam};

    #[test]
    fn exam() {
        let options = ExamOptions {};
        start_exam(options);
    }
}
