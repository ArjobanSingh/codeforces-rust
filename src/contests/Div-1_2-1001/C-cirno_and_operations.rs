use std::{
    cmp,
    io::{self, BufWriter, Read, Write},
};

fn diff_seq(nums: &Vec<i64>) -> Vec<i64> {
    let mut vec = Vec::with_capacity(nums.len() - 1);
    for i in 1..nums.len() {
        vec.push(nums[i] - nums[i - 1]);
    }
    vec
}

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

        let mut max = i64::MIN;

        let mut nums: Vec<i64> = lines
            .next()
            .expect("Error going through n")
            .split_ascii_whitespace()
            .filter_map(|it| it.parse().ok())
            .collect();

        loop {
            if nums.len() == 1 {
                max = cmp::max(max, nums[0]);
                break;
            }

            // get max sum of current array and compare.
            max = cmp::max(max, nums.iter().fold(0, |acc, cur| acc + cur));

            // get the diff sequence.
            let updated = diff_seq(&nums);
            max = cmp::max(max, updated.iter().fold(0, |acc, cur| acc + cur).abs());
            nums = updated;
        }
        writeln!(out, "{}", max).expect("Error writing empty line");
    }

    out.flush().ok();
}
