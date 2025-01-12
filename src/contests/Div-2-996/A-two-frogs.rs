use std::io::{self, BufWriter, Read, Write};

// Easier way is.
// 1. If abs distance b/w a and b is even, then the person taking first move will win
// 2. Else if odd distance, person taking 2nd move will win.
// Eg: A _ _ B. A is at 1 and B is at 4. Dist. is 3(which is odd)
// So B will win(as it is the person taking 2nd move)
// Oppposite Eg: A _ B. A is at 1 and B is at 3. Dist is 2(which is even)
// So A will win(as it will take the 1st move);

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

        let a = nums[1];
        let b = nums[2];

        if a.abs_diff(b) % 2 == 0 {
            writeln!(out, "YES").expect("Error writing newline");
        } else {
            writeln!(out, "NO").expect("Error writing newline");
        }
    }

    out.flush().ok();
}
