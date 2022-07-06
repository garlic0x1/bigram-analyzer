use crate::bigram::BigramAnalyzer;
use clap::{Parser, Subcommand};
use std::collections::HashSet;
use std::{io, io::prelude::*};

pub mod bigram;

// analyzer is case insensitive
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
    /// load from matrix file (much faster than corpus)
    #[clap(short, long)]
    matrix: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// print the probability of a word's existence
    Probability,
    /// print cleartext words from stdin{n}
    Clear {
        /// minimum probability (around 0.001 - 0.0005 seems to work well)
        #[clap(short, long)]
        score_min: Option<f32>,
        /// only print unique results
        #[clap(short, long)]
        unique: bool,
    },
    /// print hashed/encoded words from stdin{n}
    Hash {
        /// minimum probability (around 0.001 - 0.0005 seems to work well)
        #[clap(short, long)]
        score_min: Option<f32>,
        /// only print unique results
        #[clap(short, long)]
        unique: bool,
    },
    /// generate matrix file to use later without having to reread corpus 
    /// (it goes to stdout, pipe it to a file)
    Matrix {
        /// show pretty table (cannot be reused as matrix file)
        #[clap(short, long)]
        pretty: bool,
    },
}

fn main() {
    let mut unique_filter: HashSet<String> = HashSet::new();
    let args = Arguments::parse();
    let charvec = SET.chars().collect::<Vec<_>>();
    let analyzer: BigramAnalyzer;
    if args.matrix {
        analyzer = BigramAnalyzer::from_matrix(charvec, args.corpus);
    } else {
        analyzer = BigramAnalyzer::from_corpus(charvec, args.corpus);
    }

    match &args.command {
        Commands::Matrix { pretty } => {
            if *pretty {
                analyzer.print_matrix();
            } else {
                println!("{}", analyzer.store_matrix());
            }
        }
        Commands::Probability => {
            for word in io::stdin().lock().lines() {
                if let Ok(word) = word {
                    println!("{}", analyzer.weighted_slice_probability(&word));
                }
            }
        }
        Commands::Clear { score_min, unique } => {
            for word in io::stdin().lock().lines() {
                if let Ok(word) = word {
                let mut min = 0.0006;
                if let Some(m) = score_min {
                    min = *m;
                }
                    let is_clear = analyzer.weighted_slice_probability(&word) > min;
                    if is_clear && (!unique_filter.contains(&word) || !unique) {
                        println!("{}", word);
                        unique_filter.insert(word);
                    }
                }
            }
        }
        Commands::Hash { score_min, unique } => {
            for word in io::stdin().lock().lines() {
                if let Ok(word) = word {
                let mut min = 0.0006;
                if let Some(m) = score_min {
                    min = *m;
                }
                    let is_clear = analyzer.weighted_slice_probability(&word) > min;
                    if !is_clear && (!unique_filter.contains(&word) || !unique) {
                        println!("{}", word);
                        unique_filter.insert(word);
                    }
                }
            }
        }
    }
}
