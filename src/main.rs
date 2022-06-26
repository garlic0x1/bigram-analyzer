use std::collections::HashMap;
use reqwest;
use std::io::stdin;
#[macro_use]
extern crate prettytable;
use prettytable::*;
use std::io::Read;
use utf8_chars::BufReadCharsExt;

static SET: &str = "abcdefghijklmnopqrstuvwxyz";
static BREAK: &str = "\t\n !@#$%^&*()+=[]{}\\|;:'\"/?><,.`~";

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

    fn test_word(&self, word: &str) {
        let mut last: Option<char> = None;
        for c in word.chars() {
            if !self.charset.contains(&c) {
                last = None;
                continue;
            }
            if self.charset.contains(&c) {
                if let Some(l) = last {
                    let score = self.matrix.get(&l).unwrap().get(&c).unwrap();
                    println!("{}", score);
                }
                last = Some(c);
            }
        }
    }

    fn download_corpus(&self) -> Result<String, reqwest::Error> {
        println!("downloading corpus from: {}", self.corpus_url);
        let mut res = reqwest::blocking::get(self.corpus_url.clone())?;
        println!("download successful!");
            let mut body = String::new();
            res.read_to_string(&mut body);
            println!("corpus: {}", body);
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
        "https://wiki.archlinux.org/title/PipeWire".to_string(),
    );
    analyzer.analyze_corpus();
    analyzer.print();
    analyzer.test_word("qwerty");
}
