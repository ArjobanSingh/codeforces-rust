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

        let n = input[0];
        let m = input[1];

        // basically to track how many buckets can paint different no. of planks
        let mut freq: Vec<i64> = vec![0; n + 1];

        let bucket_sizes: Vec<i64> = lines
            .next()
            .expect("error reading line 2")
            .split_ascii_whitespace()
            .filter_map(|it| it.parse().ok())
            .collect();

        for size in bucket_sizes.iter() {
            freq[cmp::min(n, *size as usize)] += 1;
        }

        let mut sum = 0;
        for idx in (0..freq.len()).rev() {
            freq[idx] += sum;
            sum = freq[idx];
        }

        let mut ans: i64 = 0;
        for size in bucket_sizes.iter() {
            let mut cur_size = *size;
            loop {
                if cur_size < 0 {
                    break;
                }

                let remain_buc = (n as i64) - cur_size;
                let remain = freq[remain_buc as usize];
                if remain == 0 {
                    break;
                }

                ans += if remain_buc > *size {
                    remain
                } else {
                    remain - 1
                };
                cur_size -= 1;
            }
        }

        writeln!(out, "{ans}").expect("Error writing ans line");
    }

    out.flush().ok();
}
