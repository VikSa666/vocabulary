use rand::{self, seq::SliceRandom, thread_rng};
use vocabulary;

fn main() {
    start_exam(vocabulary::ExamOptions {});
}

pub fn start_exam(_options: vocabulary::ExamOptions) {
    let set = vocabulary::WordSet::new("transports");
    let mut words = set.get_word_refs();
    words.shuffle(&mut thread_rng());
    let mut results = vocabulary::ExamResult::new();
    for word in words.iter() {
        println!("{word}: ");
        let answer = read_from_terminal();
        match vocabulary::AskWordResult::correct_word(word, &answer) {
            vocabulary::AskWordResult::Correct => {
                results.add_one_right();
                println!("Correct!!")
            }
            vocabulary::AskWordResult::Wrong => {
                results.add_one_wrong();
                println!("That is wrong")
            }
        }
    }
    println!("{}", results.see_results())
}

fn read_from_terminal() -> String {
    let mut s = String::new();
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
    s
}
