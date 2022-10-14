use std::io::{stdin, stdout, Write};

use crate::words::{Word, WordSet};

use rand::{self, seq::SliceRandom, thread_rng};
pub struct ExamOptions {} // Todo

impl ExamOptions {
    fn _options_are_valid(&self) -> bool {
        true // todo
    }
}

pub fn start_exam(_options: ExamOptions) {
    let set = WordSet::new("transports");
    let mut words = set.get_word_refs();
    words.shuffle(&mut thread_rng());
    let mut results = ExamResult::new();
    for word in words.iter() {
        match ask_word_by_terminal(word) {
            AskWordResult::Correct => {
                results.add_one_right();
                println!("Correct!!")
            }
            AskWordResult::Wrong => {
                results.add_one_wrong();
                println!("That is wrong")
            }
        }
    }
    println!("{}", results.see_results())
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

pub struct ExamResult {
    pub right: usize,
    pub wrong: usize,
}

impl ExamResult {
    pub fn new() -> Self {
        Self { right: 0, wrong: 0 }
    }
    pub fn add_one_right(&mut self) {
        self.right += 1
    }

    pub fn add_one_wrong(&mut self) {
        self.wrong += 1
    }

    pub fn total_questions(&self) -> usize {
        self.right + self.wrong
    }

    pub fn calculate_mean(&self) -> f32 {
        (self.right as f32) / (self.total_questions() as f32)
    }
    pub fn see_results(&self) -> String {
        format!("****Exam results****\nTotal answers: {};\nRight answers: {};\nWrong answers: {};\nFinal mark: {}%", self.total_questions(), self.right, self.wrong, self.calculate_mean()*100.0)
    }
}

#[cfg(test)]
mod test {
    use crate::{exams::ExamOptions, start_exam};

    #[test]
    fn exam() {
        let options = ExamOptions {};
        start_exam(options);
    }
}
