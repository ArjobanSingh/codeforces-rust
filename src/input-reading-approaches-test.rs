use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Read, Write};
use std::time::Instant;

// Generate a large test input
fn generate_large_input(num_lines: usize, tokens_per_line: usize) -> String {
    (0..num_lines)
        .map(|_| {
            (0..tokens_per_line)
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// Approach 1: `read_to_string`
fn approach_1() -> (f64, f64) {
    let start = Instant::now();

    // Simulate reading and processing
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let lines = input.lines();

    let mut total_sum = 0.0;
    for line in lines {
        for num in line.split(" ").filter_map(|it| it.parse::<f64>().ok()) {
            total_sum += num;
        }
    }

    (start.elapsed().as_secs_f64(), total_sum)
}

// Approach 2: `Scanner`
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

fn approach_2() -> (f64, f64) {
    let start = Instant::now();

    // Simulate reading and processing
    let mut scanner = Scanner::new();

    let mut total_sum = 0.0;

    let token_size = scanner.tokens.len();

    for _ in 0..token_size {
        let num: f64 = scanner.next();
        total_sum += num; // Simulate some processing
    }

    (start.elapsed().as_secs_f64(), total_sum)
}

fn main() {
    // let num_lines = 1_000_000; // Number of lines
    // let tokens_per_line = 100; // Number of tokens per line

    // let file = File::create("test.txt").unwrap();
    // let mut writer = BufWriter::new(file);

    // Generate large input
    // let str = generate_large_input(num_lines, tokens_per_line);
    // for line in str.lines() {
    //     writeln!(writer, "{}", line).unwrap();
    // }

    // Test Approach 1
    // let (time_1, ans1) = approach_1();
    // println!("Approach 1 Time: {:.6} seconds and ans: {ans1}", time_1);

    // // Test Approach 2
    let (time_2, ans2) = approach_2();
    println!("Approach 2 Time: {:.6} seconds and ans: {ans2}", time_2);
}

// For 10,000 lines
// Approach 1: 0.012717, 0.012357, 0.013725
// Approach 2: 0.042743, 0.045444, 0.045564

// For 1M rows, Sum ans: 4950000000
// Approach 1: 1.103472, 1.250184, 1.257301
// Approach 2: 4.118857, 4.479919, 4.274032

// Just for reading
// Approach 1: 0.103140249, 0.104034317, 0.104265528
// Approach 2: 2.864570, 2.911595617, 3.0546779
