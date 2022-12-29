use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashSet;
use std::fs;

use calamine::{open_workbook, Error, Reader, Xlsx};

fn main() {
    let words = read_excel();
    let cathegories = transform(&words);
    let serialized = serde_json::to_string(&cathegories).unwrap();
    let path = format!("{}/resources/vocab.json", env!("CARGO_MANIFEST_DIR"));
    let contents = serialized;
    fs::write(path, contents).unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RealWord {
    russian: String,
    spanish: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Cathegory {
    name: String,
    words: Vec<RealWord>,
}

impl Into<Cathegory> for String {
    fn into(self) -> Cathegory {
        Cathegory {
            name: self,
            words: vec![],
        }
    }
}

fn transform(words: &Vec<Word>) -> Vec<Cathegory> {
    let mut cathegory_buf = HashSet::new();
    for word in words {
        cathegory_buf.insert(word.cathegory().to_string());
    }
    let mut cathegories = Vec::new();
    for cathegory in cathegory_buf {
        let words = words.clone();
        cathegories.push(Cathegory {
            name: cathegory.clone(),
            words: words
                .clone()
                .iter()
                .filter(|w| w.cathegory == cathegory)
                .map(|w| RealWord {
                    russian: w.russian.clone(),
                    spanish: w
                        .spanish
                        .clone()
                        .split(", ")
                        .map(|c| c.to_string())
                        .collect(),
                })
                .collect::<Vec<RealWord>>(),
        })
    }
    cathegories
}

#[derive(Debug, Clone)]
struct Word {
    russian: String,
    spanish: String,
    cathegory: String,
}

impl Word {
    fn new(russian: &str, spanish: &str, cathegory: &str) -> Self {
        Self {
            russian: russian.to_string(),
            spanish: spanish.to_string(),
            cathegory: cathegory.to_string(),
        }
    }

    fn cathegory(&self) -> &str {
        &self.cathegory
    }
}

fn read_excel() -> Vec<Word> {
    let path = format!("{}/resources/vocab.xlsx", env!("CARGO_MANIFEST_DIR"));
    println!("{}", env!("CARGO_MANIFEST_DIR"));
    let mut workbook: Xlsx<_> =
        open_workbook(&path).expect(format!("Could not open {}", path.to_string()).as_str());
    let range = workbook
        .worksheet_range("Vocabulario")
        .ok_or(Error::Msg("Cannot find 'Vocabulario'"))
        .unwrap()
        .unwrap();
    let mut words = Vec::new();
    for row in range.rows() {
        words.push(Word::new(
            row[0].to_string().as_str(),
            row[1].to_string().as_str(),
            row[2].to_string().as_str(),
        ))
    }
    words
}

#[cfg(test)]
mod test {
    use calamine::{open_workbook, Error, RangeDeserializerBuilder, Reader, Xlsx};

    #[test]
    #[ignore]
    fn example() -> Result<(), Error> {
        Ok(())
    }
}
