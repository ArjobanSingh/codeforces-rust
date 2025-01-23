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

    let t: i32 = lines
        .next()
        .expect("Error reading t")
        .trim()
        .parse()
        .expect("Error in parsing t");

    for _ in 0..t {
        let mut result: Vec<i32> = vec![0; 5];
        for (idx, num) in lines
            .next()
            .expect("Error going through n")
            .split_ascii_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .enumerate()
        {
            if idx >= 2 {
                result[idx + 1] = num;
            } else {
                result[idx] = num;
            }
        }

        let mut ans = 0;
        result[2] = result[1] + result[0];
        for i in 0usize..=2 {
            if result[i] + result[i + 1] == result[i + 2] {
                ans += 1;
            }
        }

        result[2] = result[3] - result[1];

        let mut ans_2 = 0;
        for i in 1usize..=2 {
            if result[i] + result[i + 1] == result[i + 2] {
                ans_2 += 1;
            }
        }

        writeln!(out, "{}", cmp::max(ans, ans_2)).expect("Error writing empty line");
    }

    out.flush().ok();
}
