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
        let input: Vec<usize> = lines
            .next()
            .expect("error reading line")
            .split_ascii_whitespace()
            .filter_map(|it| it.parse().ok())
            .collect();

        let mut n = input[0];
        let k = input[1];

        let mut ans = 0;

        let is_even = n % 2 == 0;
        let (largest_even, largest_odd) = if k % 2 == 0 { (k, k - 1) } else { (k - 1, k) };

        // convert to even by subtracting largest odd
        if !is_even {
            ans += 1;
            n -= largest_odd;
        }

        // once even. just divide by largest even and add +1 if not completely divisble
        if n > 0 {
            let max_k = cmp::min(largest_even, n);
            ans += n / max_k;
            if n % max_k != 0 {
                ans += 1;
            }
        }

        writeln!(out, "{ans}").expect("Error writing ans line");
    }

    out.flush().ok();
}
