pub mod error;
mod exams;
pub mod words;

pub use {
    exams::ExamOptions,
    words::{new_word, Word, WordSet},
};
