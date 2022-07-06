use prettytable::*;
use reqwest;
use std::collections::HashMap;

pub struct BigramAnalyzer {
    matrix: HashMap<char, HashMap<char, f32>>,
    corpus_filename: Option<String>,
    matrix_filename: Option<String>,
    charset: Vec<char>,
}

impl BigramAnalyzer {
    pub fn from_corpus(charset: Vec<char>, corpus_filename: String) -> Self {
        Self {
            charset,
            corpus_filename: Some(corpus_filename),
            matrix_filename: None,
            matrix: HashMap::new(),
        }
    }

    pub fn from_matrix(charset: Vec<char>, matrix_filename: String) -> Self {
        Self {
            charset,
            corpus_filename: None,
            matrix_filename: Some(matrix_filename),
            matrix: HashMap::new(),
        }
    }

    pub fn is_word_cleartext(&self, word: &str, min_score: Option<f32>) -> bool {
        let min: f32;
        match min_score {
            Some(m) => min = m,
            None => {
                min = 0.0001;
            }
        }
        
        self.weighted_slice_probability(word) > min
    }

    fn download_corpus(&self) -> Result<String, reqwest::Error> {
        let res = reqwest::blocking::get(self.corpus_filename.clone().unwrap())?;
        let body = res.text().expect("Failed to read downloaded corpus");
        Ok(body)
    }

    fn read_local(&self) -> Result<String, std::io::Error> {
        let s = std::fs::read_to_string(self.corpus_filename.clone().unwrap())?;
        Ok(s)
    }

    pub fn load_matrix(&mut self) {
        let s = std::fs::read_to_string(self.matrix_filename.clone().unwrap()).expect("no such file");

        let mut lines = s.lines();
        let mut charset = Vec::new();
        let set = lines.next();
        for c in set.unwrap().chars() {
            if c != ',' {
                charset.push(c);
            }
        }

        for a in charset.iter() {
            let mut bmap = HashMap::new();
            let mut set = lines.next().unwrap().split(',');
            for b in charset.iter() {
                let strval = set.next().unwrap().to_string();
                let val: f32 = strval.parse().unwrap();
                bmap.insert(*b, val);
            }

            self.matrix.insert(*a, bmap);
        }
    }

    pub fn store_matrix(&self) -> String {
        let mut store = String::new();
        for c in self.charset.iter() {
            store.push(*c);
            store.push(',');
        }
        store.push('\n');

        for a in self.charset.iter() {
            for b in self.charset.iter() {
                let val = *self.matrix.get(a).unwrap().get(b).unwrap();
                store.push_str(&val.to_string());
                store.push(',');
            }
            store.push('\n');
        }

        store
    }

    pub fn analyze_corpus(&mut self) {
        for i in self.charset.iter() {
            let mut inner: HashMap<char, f32> = HashMap::new();
            for j in self.charset.iter() {
                inner.insert(*j, 0.0);
            }
            self.matrix.insert(*i, inner);
        }

        let corpus: String;

        let filename = self.corpus_filename.clone().unwrap();
        if filename.starts_with("http://")
            || filename.starts_with("https://")
        {
            corpus = self.download_corpus().expect("Failed to download corpus");
        } else {
            corpus = self
                .read_local()
                .expect(&format!("no such file or URL {}", filename));
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
                    *cell += 1.0;
                    counter += 1;
                }
                last = Some(c);
            }
        }

        for (_,map) in self.matrix.iter_mut() {
            for (_, val) in map.iter_mut() {
                *val /= counter as f32;
            }
        }
    }

    /// length independent probability
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

                    sum += *value;
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

    /// probability that a word exists based on corpus data
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

                    probability *= *value;
                }
                last = Some(c);
            }
        }
        probability
    }

    /// pretty printing with a table ( too small for terminal though )
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
                let value = *value;
                row.add_cell(Cell::new(&value.to_string()));
            }
            table.add_row(row);
        }
        table.printstd();
    }
}
