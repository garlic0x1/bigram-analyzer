use clap::{Parser, Subcommand};
use std::collections::HashSet;
use std::{io, io::prelude::*};
use crate::bigram::BigramAnalyzer;

pub mod bigram;

static SET: &str = "abcdefghijklmnopqrstuvwxyz1234567890";


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
