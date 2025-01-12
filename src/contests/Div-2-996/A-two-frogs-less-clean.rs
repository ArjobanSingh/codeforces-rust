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
        let nums: Vec<i32> = lines
            .next()
            .expect("Error reading line")
            .split_ascii_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let n = nums[0];
        let mut a = nums[1];
        let mut b = nums[2];

        loop {
            // initial direction
            let to_right = a < b;
            if to_right {
                if a + 1 != b {
                    a = a + 1
                } else if a - 1 >= 1 {
                    a = a - 1;
                } else {
                    writeln!(out, "NO").expect("Error writing newline");
                    break;
                }
            } else {
                if a - 1 != b {
                    a = a - 1
                } else if a + 1 <= n {
                    a = a + 1;
                } else {
                    writeln!(out, "NO").expect("Error writing newline");
                    break;
                }
            }

            // check for bob
            let to_right = !to_right;
            if to_right {
                if b + 1 != a {
                    b = b + 1
                } else if b - 1 >= 1 {
                    b = b - 1;
                } else {
                    // BOB is losing, so print yes
                    writeln!(out, "YES").expect("Error writing newline");
                    break;
                }
            } else {
                if b - 1 != a {
                    b = b - 1
                } else if b + 1 <= n {
                    b = b + 1;
                } else {
                    writeln!(out, "YES").expect("Error writing newline");
                    break;
                }
            }
        }
    }

    out.flush().ok();
}
