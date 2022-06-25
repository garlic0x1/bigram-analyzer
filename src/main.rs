use std::collections::HashMap;
use std::io::stdin;
use utf8_chars::BufReadCharsExt;

static SET: &str = "abcdefghijklmnopqrstuvwxyz1234567890";
static BREAK: &str = "\t\n !@#$%^&*()+=[]{}\\|;:'\"/?><,.`~";

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
                println!("accessing: {}, {}", l, c);
                let cell = matrix
                    .get_mut(&l)
                    .expect("no row")
                    .get_mut(&c)
                    .expect("no cell");
                *cell += 1;
            }
            last = Some(c);
        }
        println!("{}", c);
    }
    for (k, v) in matrix.iter() {
        print!("{}: ", k);
        for (k, v) in v.iter() {
            print!("{}={} ", k, v);
        }
    }
}
