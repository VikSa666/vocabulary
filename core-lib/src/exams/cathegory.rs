pub enum QuestionByCathegory {
    All,
    Selection(Vec<String>),
}

impl QuestionByCathegory {
    pub fn new_from_string(cathegory: &str) -> Self {
        match cathegory {
            "all" | "every cathegory" | "all cathegories" => Self::All,
            other => {
                let other = other.split(", ").collect::<String>();
                let other = other
                    .split_ascii_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                Self::Selection(other)
            }
        }
    }
}
