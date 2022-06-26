use reqwest;
use std::collections::HashMap;
use std::io::stdin;
#[macro_use]
extern crate prettytable;
use prettytable::*;
use std::io::Read;
use utf8_chars::BufReadCharsExt;

static SET: &str = "abcdefghijklmnopqrstuvwxyz1234567890-+_";

struct BigramAnalyzer {
    matrix: HashMap<char, HashMap<char, u32>>,
    corpus_url: String,
    charset: Vec<char>,
}

impl BigramAnalyzer {
    fn new(charset: Vec<char>, corpus_url: String) -> Self {
        Self {
            charset,
            corpus_url,
            matrix: HashMap::new(),
        }
    }

    fn is_word_cleartext(&self, word: &str, min_score: u32, max_occurrences: u32) -> bool {
        let mut occurrences: u32 = 0;
        let mut last: Option<char> = None;
        for c in word.chars() {
            let mut c = c;
            let ascii = c as u8;
            if ascii > 64 && ascii < 91 {
               c = (ascii + 32) as char; 
            }
            if !self.charset.contains(&c) {
                last = None;
                continue;
            }
            if self.charset.contains(&c) {
                if let Some(l) = last {
                    let score = self.matrix.get(&l).unwrap().get(&c).unwrap();
                    if score < &min_score {
                        occurrences += 1;
                    }
                }
                last = Some(c);
            }
        }
        occurrences < max_occurrences
    }

    fn download_corpus(&self) -> Result<String, reqwest::Error> {
        println!("downloading corpus from: {}", self.corpus_url);
        let mut res = reqwest::blocking::get(self.corpus_url.clone())?;
        println!("download successful!");
        let mut body = String::new();
        res.read_to_string(&mut body);
        Ok(body)
    }

    fn analyze_corpus(&mut self) {
        for i in self.charset.iter() {
            let mut inner: HashMap<char, u32> = HashMap::new();
            for j in self.charset.iter() {
                inner.insert(*j, 0);
            }
            self.matrix.insert(*i, inner);
        }
        let mut last: Option<char> = None;
        let corpus = self.download_corpus().unwrap();
        for c in corpus.chars() {
            let mut c = c;
            let ascii = c as u8;
            if ascii > 64 && ascii < 91 {
               c = (ascii + 32) as char; 
            }
            if !self.charset.contains(&c) {
                last = None;
                continue;
            }
            if self.charset.contains(&c) {
                if let Some(l) = last {
                    let cell = self
                        .matrix
                        .get_mut(&l)
                        .expect("no row")
                        .get_mut(&c)
                        .expect("no cell");
                    *cell += 1;
                }
                last = Some(c);
            }
        }
    }

    fn print(&self) {
        let mut table = Table::new();
        let mut start_row = Row::new(Vec::new());
        start_row.add_cell(Cell::new("MATRIX"));
        for c in self.charset.iter() {
            start_row.add_cell(Cell::new(c.to_string().as_str()));
        }
        table.add_row(start_row);
        for c in SET.chars() {
            let mut row = Row::new(Vec::new());
            row.add_cell(Cell::new(c.to_string().as_str()));
            for inner in SET.chars() {
                let value = self.matrix.get(&c).unwrap().get(&inner).unwrap();
                row.add_cell(Cell::new(&value.to_string()));
            }
            table.add_row(row);
        }
        table.printstd();
    }
}

fn main() {
    let charvec = SET.chars().collect::<Vec<_>>();
    let mut analyzer = BigramAnalyzer::new(
        charvec,
        "https://github.com/garlic0x1".to_string(),
    );
    analyzer.analyze_corpus();
    analyzer.print();
    let test_list = vec!["animal", "bvfks", "snowball", "bptpvojlk", "realword"];
    for word in test_list {
        let is_clear = analyzer.is_word_cleartext(word, 30, 1);
        if is_clear {
            println!("{} is cleartext", word);
        } else {
            println!("{} is encoded", word);
        }
    }
}
