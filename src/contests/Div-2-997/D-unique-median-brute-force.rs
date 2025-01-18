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
        let mut result = 0;
        let n: i32 = lines
            .next()
            .expect("Error going through n")
            .parse()
            .unwrap();

        let nums: Vec<i32> = lines
            .next()
            .expect("error reading n")
            .split_ascii_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        for i in 0..n {
            for j in i..n {
                let len = j - i + 1;
                if len % 2 != 0 {
                    result += 1;
                    continue;
                }

                let mut sub = nums[i as usize..=j as usize].to_owned();
                sub.sort();
                let mid = sub.len() / 2;
                if sub[mid] == sub[mid - 1] {
                    result += 1;
                }
            }
        }

        writeln!(out, "{result}").expect("Error writing ans");
    }

    out.flush().ok();
}
