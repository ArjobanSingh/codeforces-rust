use std::io::{self, BufWriter, Read, Write};

// TC: O(M * (lN to rN)) // where L and R are ith idx of N
// l and R can be full range of N in each fight, so this boils down to
// TC: O(N * M)
fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();
    let mut out = BufWriter::new(io::stdout().lock());

    let nums: Vec<i32> = lines
        .next()
        .expect("Error reading line")
        .split_ascii_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

    let n = nums[0];
    let m = nums[1];
    let mut result = vec![0; (n + 1) as usize];

    for _ in 0..m {
        let fight: Vec<i32> = lines
            .next()
            .expect("Error reading line")
            .split_ascii_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let l = fight[0];
        let r = fight[1];
        let winner = fight[2];

        for cur in l..=r {
            if cur == winner {
                continue;
            }
            if result[cur as usize] == 0 {
                result[cur as usize] = winner;
            }
        }
    }

    result.iter().skip(1).for_each(|&winner| {
        write!(out, "{} ", winner).expect("Error writing");
    });

    writeln!(out).expect("Error writing newline");
    out.flush().ok();
}
