use prettytable::*;
use reqwest;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub struct BigramAnalyzer {
    matrix: HashMap<char, HashMap<char, u32>>,
    corpus_filename: String,
    corpus_size: u32,
    charset: Vec<char>,
}

impl BigramAnalyzer {
    pub fn new(charset: Vec<char>, corpus_filename: String) -> Self {
        Self {
            charset,
            corpus_size: 0,
            corpus_filename,
            matrix: HashMap::new(),
        }
    }

    pub fn is_word_cleartext(&self, word: &str, min_score: Option<u32>, max_occurrences: u32) -> bool {
        let min: u32;
        match min_score {
            Some(m) => min = m,
            None => {
                min = self.corpus_size / 10_000;
            }
            //println!("size threshold set to {} occurrences", min);},
        }
        let mut occurrences: u32 = 0;
        let mut last: Option<char> = None;
        for c in word.chars() {
            let mut c = c;
            // standardize case
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
                    if score < &min {
                        occurrences += 1;
                    }
                }
                last = Some(c);
            }
        }
        occurrences < max_occurrences
    }

    fn download_corpus(&self) -> Result<String, reqwest::Error> {
        let res = reqwest::blocking::get(self.corpus_filename.clone())?;
        let body = res.text().expect("Failed to read downloaded corpus");
        Ok(body)
    }

    fn read_local(&self) -> Result<String, std::io::Error> {
        let s = std::fs::read_to_string(self.corpus_filename.clone())?;
        Ok(s)
    }

    pub fn analyze_corpus(&mut self) {
        for i in self.charset.iter() {
            let mut inner: HashMap<char, u32> = HashMap::new();
            for j in self.charset.iter() {
                inner.insert(*j, 0);
            }
            self.matrix.insert(*i, inner);
        }

        let corpus: String;
        if self.corpus_filename.clone().starts_with("http://")
            || self.corpus_filename.starts_with("https://")
        {
            corpus = self.download_corpus().expect("Failed to download corpus");
        } else {
            corpus = self
                .read_local()
                .expect(&format!("no such file or URL {}", self.corpus_filename));
        }

        let mut last: Option<char> = None;
        let mut counter: u32 = 0;
        for ch in corpus.chars() {
            // standardize case
            let mut c = ch;
            let ascii = c as u8;
            if ascii > 64 && ascii < 91 {
                c = (ascii + 32) as char;
            } else {
                c = ch;
            }

            if !self.charset.contains(&c) {
                last = None;
            } else {
                if let Some(l) = last {
                    let cell = self
                        .matrix
                        .get_mut(&l)
                        .expect("no row")
                        .get_mut(&c)
                        .expect("no cell");
                    *cell += 1;
                    counter += 1;
                }
                last = Some(c);
            }
        }
        self.corpus_size = counter;
    }

    pub fn weighted_slice_probability(&self, string: &str) -> f32 {
        let mut sum = 0.0;
        let mut counter = 0;
        let mut last: Option<char> = None;
        for ch in string.chars() {
            // standardize case
            let mut c = ch;
            let ascii = c as u8;
            if ascii > 64 && ascii < 91 {
                c = (ascii + 32) as char;
            } else {
                c = ch;
            }

            if !self.charset.contains(&c) {
                last = None;
            } else {
                if let Some(l) = last {
                    let value = self
                        .matrix
                        .get(&l)
                        .expect("no row")
                        .get(&c)
                        .expect("no cell");

                    sum += *value as f32 / self.corpus_size as f32;
                }
                last = Some(c);
            }
            counter += 1;
        }

        if counter == 1 {
            1.0 
        } else {
            sum / counter as f32
        }
    }

    pub fn slice_probability(&self, string: &str) -> f32 {
        let mut probability = 1.0;
        let mut last: Option<char> = None;
        for ch in string.chars() {
            // standardize case
            let mut c = ch;
            let ascii = c as u8;
            if ascii > 64 && ascii < 91 {
                c = (ascii + 32) as char;
            } else {
                c = ch;
            }

            if !self.charset.contains(&c) {
                last = None;
            } else {
                if let Some(l) = last {
                    let value = self
                        .matrix
                        .get(&l)
                        .expect("no row")
                        .get(&c)
                        .expect("no cell");

                    probability *= *value as f32 / self.corpus_size as f32;
                }
                last = Some(c);
            }
        }
        probability
    }

    pub fn print_matrix(&self) {
        let mut table = Table::new();
        let mut start_row = Row::new(Vec::new());
        start_row.add_cell(Cell::new("MATRIX"));
        for c in self.charset.iter() {
            start_row.add_cell(Cell::new(c.to_string().as_str()));
        }
        table.add_row(start_row);
        for c in &self.charset {
            let mut row = Row::new(Vec::new());
            row.add_cell(Cell::new(c.to_string().as_str()));
            for inner in &self.charset {
                let value = self.matrix.get(&c).unwrap().get(&inner).unwrap();
                let value = *value as f32 / self.corpus_size as f32;
                row.add_cell(Cell::new(&value.to_string()));
            }
            table.add_row(row);
        }
        table.printstd();
    }
}
