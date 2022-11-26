pub mod cathegory;
pub mod exam;

use crate::words;
pub struct ExamOptions {
    pub check_all_answer: bool,
    pub cathegory: cathegory::QuestionByCathegory,
} // Todo

impl Default for ExamOptions {
    fn default() -> Self {
        Self {
            check_all_answer: Default::default(),
            cathegory: cathegory::QuestionByCathegory::All,
        }
    }
}

impl ExamOptions {
    fn _options_are_valid(&self) -> bool {
        true // todo
    }
}

pub enum AskWordResult {
    Correct,
    Wrong,
}

impl AskWordResult {
    pub fn correct_word(word: &words::Word, answer: &str) -> Self {
        if word.translations.contains(&answer.to_string()) {
            Self::Correct
        } else {
            Self::Wrong
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
