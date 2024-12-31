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

    let mut ans: String = String::new();
    let str: String = scanner.next();

    for &byte in str.as_bytes() {
        let ch = byte as char;
        let lower_ch = ch.to_ascii_lowercase();
        match lower_ch {
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' => (),
            _ => {
                ans.push('.');
                ans.push(lower_ch);
            }
        }
    }

    writeln!(out, "{ans}").ok();
}
