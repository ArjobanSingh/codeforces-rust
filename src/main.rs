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
    let n: usize = scanner.next();

    let mut ans = 0;

    writeln!(out, "{ans}").ok();
}
