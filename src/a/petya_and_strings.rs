use std::{
    cmp::Ordering,
    collections::VecDeque,
    io::{self, BufRead, BufWriter, Write},
};

#[derive(Default)]
struct Scanner {
    tokens: VecDeque<String>,
}

impl Scanner {
    pub fn new() -> Self {
        let stdin = io::stdin();
        let tokens: VecDeque<String> = stdin.lock().lines().filter_map(|line| line.ok()).collect();
        Self { tokens }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        self.tokens
            .pop_front()
            .expect("No more tokens")
            .parse()
            .ok()
            .expect("Failed parse")
    }
}

fn main() {
    let mut scanner = Scanner::new();
    let out = &mut BufWriter::new(io::stdout());

    let line1: String = scanner.next();
    let line2: String = scanner.next();

    let line2_bytes = line2.as_bytes();
    for (idx, &ch) in line1.as_bytes().iter().enumerate() {
        let ch = (ch as char).to_ascii_lowercase();
        let other_ch = (line2_bytes[idx] as char).to_ascii_lowercase();
        match ch.cmp(&other_ch) {
            Ordering::Less => {
                writeln!(out, "-1").ok();
                return;
            }
            Ordering::Greater => {
                writeln!(out, "1").ok();
                return;
            }
            _ => (),
        }
    }
    writeln!(out, "0").ok();
}
