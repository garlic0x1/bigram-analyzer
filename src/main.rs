use clap::{Parser, Subcommand};
use std::collections::HashSet;
use prettytable::*;
use reqwest;
use std::collections::HashMap;
use std::{io, io::prelude::*};

static SET: &str = "abcdefghijklmnopqrstuvwxyz1234567890-+_";

struct BigramAnalyzer {
    matrix: HashMap<char, HashMap<char, u32>>,
    corpus_filename: String,
    charset: Vec<char>,
}

impl BigramAnalyzer {
    fn new(charset: Vec<char>, corpus_filename: String) -> Self {
        Self {
            charset,
            corpus_filename,
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
        let mut res = reqwest::blocking::get(self.corpus_filename.clone())?;
        let mut body = String::new();
        res.read_to_string(&mut body).expect("Failed to read downloaded corpus");
        Ok(body)
    }

    fn read_local(&self) -> Result<String, std::io::Error> {
        let s = std::fs::read_to_string(self.corpus_filename.clone())?;
        Ok(s)
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

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Arguments {
    #[clap(subcommand)]
    command: Commands,
    /// local file or URL to generate matrix with
    #[clap(value_parser)]
    corpus: String,
}

#[derive(Subcommand)]
enum Commands {
    /// print cleartext words from stdin{n}
    Clear {
        /// minimum occurence score for "common bigraph"
        #[clap(short, long, default_value = "10")]
        score_min: u32,
        /// n rare bigraphs to be encoded
        #[clap(short, long, default_value = "1")]
        occurrences_max: u32,
        /// only print unique results
        #[clap(short, long)]
        unique: bool,
    },
    /// print hashed/encoded words from stdin{n}
    Hash {
        /// minimum occurence score for "common bigraph"
        #[clap(short, long, default_value = "10")]
        score_min: u32,
        /// n rare bigraphs to be encoded
        #[clap(short, long, default_value = "1")]
        occurrences_max: u32,
        /// only print unique results
        #[clap(short, long)]
        unique: bool,
    },
    /// print occurrence matrix{n}
    Matrix,
}

fn main() {
    let mut unique_filter: HashSet<String> = HashSet::new();
    let args = Arguments::parse();
    let charvec = SET.chars().collect::<Vec<_>>();
    let mut analyzer = BigramAnalyzer::new(charvec, args.corpus);
    analyzer.analyze_corpus();

    match &args.command {
        Commands::Matrix => {
            analyzer.print();
        }
        Commands::Clear { score_min, occurrences_max, unique } => {
            for word in io::stdin().lock().lines() {
                if let Ok(word) = word {
                    let is_clear = analyzer.is_word_cleartext(&word, *score_min, *occurrences_max);
                    if is_clear && (!unique_filter.contains(&word) || !unique) {
                        println!("{}", word);
                        unique_filter.insert(word);
                    }
                }
            }
        }
        Commands::Hash { score_min, occurrences_max, unique } => {
            for word in io::stdin().lock().lines() {
                if let Ok(word) = word {
                    let is_clear = analyzer.is_word_cleartext(&word, *score_min, *occurrences_max);
                    if !is_clear && (!unique_filter.contains(&word) || !unique) {
                        println!("{}", word);
                        unique_filter.insert(word);
                    }
                }
            }
        }
    }
}
