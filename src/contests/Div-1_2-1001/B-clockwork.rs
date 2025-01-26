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
    let mut out = BufWriter::new(io::stdout().lock());

    let t: i64 = lines
        .next()
        .expect("Error reading t")
        .trim()
        .parse()
        .expect("Error in parsing t");

    for _ in 0..t {
        let n: usize = lines
            .next()
            .expect("error reading n")
            .parse()
            .expect("error parsing n");

        let mut failed = false;

        for (idx, it) in lines
            .next()
            .expect("Error going through n")
            .split_ascii_whitespace()
            .enumerate()
        {
            if let Ok(num) = it.parse::<usize>() {
                let dist_from_far_end = cmp::max(idx.abs_diff(0), idx.abs_diff(n - 1)) * 2;
                if dist_from_far_end >= num {
                    failed = true;
                    break;
                }
            }
        }

        writeln!(out, "{}", if failed { "NO" } else { "YES" }).expect("Error writing empty line");
    }

    out.flush().ok();
}
