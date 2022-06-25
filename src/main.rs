use std::collections::HashMap;
use std::io::stdin;
#[macro_use] extern crate prettytable;
use prettytable::*;
use utf8_chars::BufReadCharsExt;

static SET: &str = "abcdefghijklmnopqrstuvwxyz";
static BREAK: &str = "\t\n !@#$%^&*()+=[]{}\\|;:'\"/?><,.`~";

/*
struct BigramAnalyzer {
    matrix: HashMap<char, HashMap<char, u32>>,
}
*/

fn main() {
    let mut matrix: HashMap<char, HashMap<char, u32>> = HashMap::new();
    for i in SET.chars() {
        let mut inner: HashMap<char, u32> = HashMap::new();
        for j in SET.chars() {
            inner.insert(j, 0);
        }
        matrix.insert(i, inner);
    }
    let mut last: Option<char> = None;
    for c in stdin().lock().chars().map(|x| x.unwrap()) {
        if BREAK.contains(c) {
            last = None;
            continue;
        }
        if SET.contains(c) {
            if let Some(l) = last {
                let cell = matrix
                    .get_mut(&l)
                    .expect("no row")
                    .get_mut(&c)
                    .expect("no cell");
                *cell += 1;
            }
            last = Some(c);
        }
    }
    
    let mut table = Table::new();;
    let mut start_row = Row::new(Vec::new());
    start_row.add_cell(Cell::new("MATRIX"));
    for c in SET.chars() {
        start_row.add_cell(Cell::new(c.to_string().as_str()));
    }
    table.add_row(start_row);
    for c in SET.chars() {
        let mut row = Row::new(Vec::new());
        row.add_cell(Cell::new(c.to_string().as_str()));
    for inner in SET.chars() {
        let value = matrix.get(&c).unwrap().get(&inner).unwrap();
        row.add_cell(Cell::new(&value.to_string()));
    }
    table.add_row(row);
    }
    table.printstd();
}
