use std::{
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
        let mut tokens = VecDeque::new();

        for line in stdin.lock().lines() {
            for token in line.expect("Failed to read line").split_ascii_whitespace() {
                tokens.push_back(token.to_owned());
            }
        }

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
    let n: i32 = scanner.next();

    let mut even_count = 0;
    let mut odd_count = 0;

    let mut first_even_idx = -1;
    let mut first_odd_idx = -1;

    for idx in 0..n {
        let num: i32 = scanner.next();
        if num % 2 == 0 {
            even_count += 1;
            if first_even_idx < 0 {
                first_even_idx = idx + 1;
            }
        } else {
            odd_count += 1;
            if first_odd_idx < 0 {
                first_odd_idx = idx + 1;
            }
        }
    }

    if even_count < odd_count {
        writeln!(out, "{first_even_idx}").ok();
    } else {
        writeln!(out, "{first_odd_idx}").ok();
    }
}
