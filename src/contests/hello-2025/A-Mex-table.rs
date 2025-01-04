use std::{
    cmp,
    io::{self, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();
    let t: usize = lines
        .next()
        .expect("error reading t")
        .trim()
        .parse()
        .expect("Error converting t");

    let mut out = BufWriter::new(io::stdout().lock());

    for _ in 0..t {
        let nums: Vec<u64> = lines
            .next()
            .expect("Error reading line")
            .split_ascii_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let n = nums[0];
        let m = nums[1];
        writeln!(out, "{}", cmp::max(n, m) + 1).expect("Error writing");
    }

    out.flush().ok();
}
