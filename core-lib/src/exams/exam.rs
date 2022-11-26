use std::collections::HashMap;

use crate::Word;

pub struct Exam<'a> {
    questions: HashMap<&'a str, Word>,
}

impl<'a> Exam<'a> {
    pub fn new(hashmap: HashMap<&'a str, Word>) -> Self {
        Self { questions: hashmap }
    }
}
