use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();
    let mut out = BufWriter::new(io::stdout().lock());

    let t: i32 = lines
        .next()
        .expect("Error reading t")
        .trim()
        .parse()
        .expect("Error in parsing t");

    for _ in 0..t {
        let n: i32 = lines
            .next()
            .expect("Error reading t")
            .trim()
            .parse()
            .expect("Error in parsing t");

        let current: Vec<i32> = lines
            .next()
            .expect("Error reading line")
            .split_ascii_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let needed: Vec<i32> = lines
            .next()
            .expect("Error reading line")
            .split_ascii_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let mut diff_and_idx = (i32::MIN, None);
        for idx in 0..current.len() {
            let diff = needed[idx] - current[idx];
            if diff > diff_and_idx.0 {
                diff_and_idx.0 = diff;
                diff_and_idx.1 = Some(idx);
            }
        }

        let (max_diff, idx) = diff_and_idx;
        if let Some(max_diff_idx) = idx {
            if max_diff > 0 {
                let is_possible = current.iter().enumerate().all(|(item_idx, num)| {
                    if max_diff_idx == item_idx {
                        return true;
                    }
                    num - max_diff >= needed[item_idx]
                });
                writeln!(out, "{}", if is_possible { "YES" } else { "NO" })
                    .expect("Error writing ans");
            } else {
                writeln!(out, "YES").expect("Error writing ans in else");
            }
        }
    }

    out.flush().ok();
}
