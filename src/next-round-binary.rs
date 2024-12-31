use std::{
    collections::VecDeque,
    io::{self, BufRead, BufWriter, Write},
};

#[derive(Default)]
struct Scanner {
    buffer: VecDeque<String>,
    items: Vec<u8>,
}

impl Scanner {
    pub fn new() -> Self {
        let stdin = io::stdin();
        let mut buffer = VecDeque::new();
        let mut items: Vec<u8> = Vec::new();

        let mut read_count = 0;
        for line in stdin.lock().lines() {
            for token in line.expect("Failed to read line").split_ascii_whitespace() {
                if read_count < 2 {
                    buffer.push_back(token.to_owned());
                    read_count += 1;
                    if read_count == 2 {
                        items = Vec::with_capacity(buffer.pop_front().unwrap().parse().unwrap());
                    }
                } else {
                    items.push(token.parse().expect("Issue is parsing u8"));
                }
            }
        }

        Self { buffer, items }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        self.buffer
            .pop_front()
            .expect("Empty buffer")
            .parse()
            .ok()
            .expect("Failed parse")
    }
}

fn main() {
    let mut scanner = Scanner::new();
    let out = &mut BufWriter::new(io::stdout());

    let k: usize = scanner.next();

    let items = scanner.items;
    let kth_score = items[k - 1];

    let mut l: i32 = 0;
    let mut r: i32 = (items.len() as i32) - 1;

    // Find the first non zero bigger element to the left side
    if kth_score == 0 {
        while l <= r {
            let mid = l + (r - l) / 2;
            if items[mid as usize] <= 0 {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
    } else {
        // Find the last index which share same element as kth_element
        while l <= r {
            let mid = l + (r - l) / 2;

            if items[mid as usize] >= kth_score {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
    }

    writeln!(out, "{}", if r < 0 { 0 } else { r + 1 }).ok();
}
