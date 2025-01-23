use std::io::{self, BufWriter, Read, Write};

const MAX_N: usize = 200000;

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
        let arr: Vec<usize> = lines
            .next()
            .expect("Error going through n")
            .split_ascii_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let target = arr[1];

        let mut frq = vec![0; MAX_N + 1];

        let mut ans = 0;
        for num in lines
            .next()
            .expect("Error going through n nums")
            .split_ascii_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
        {
            let other = target as i32 - num;
            if other > 0 && other as usize <= MAX_N && frq[other as usize] > 0 {
                ans += 1;
                frq[other as usize] -= 1;
            } else {
                frq[num as usize] += 1;
            }
        }

        writeln!(out, "{ans}").expect("Error writing empty line");
    }

    out.flush().ok();
}
