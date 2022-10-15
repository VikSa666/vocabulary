pub mod error;
mod exams;
pub mod words;

pub use {
    exams::{AskWordResult, ExamOptions, ExamResult},
    words::{new_word, Word, WordSet},
};
