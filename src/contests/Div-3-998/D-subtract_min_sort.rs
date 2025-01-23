use std::io::{self, BufWriter, Read, Write};

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
        lines.next().expect("error reading n");
        let mut arr: Vec<usize> = lines
            .next()
            .expect("Error going through n")
            .split_ascii_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let mut is_possible = true;
        for i in 0..arr.len() - 1 {
            if arr[i + 1] < arr[i] {
                is_possible = false;
                break;
            }

            arr[i + 1] -= arr[i];
            arr[i] = 0;
        }

        writeln!(out, "{}", if is_possible { "YES" } else { "NO" })
            .expect("Error writing empty line");
    }

    out.flush().ok();
}
