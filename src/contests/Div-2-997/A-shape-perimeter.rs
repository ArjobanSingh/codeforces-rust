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
            .expect("error reading n")
            .split_ascii_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let n = nums[0];
        let m = nums[1];

        let mut total_perimeter = m * 4 * n;

        // skip first iteration
        lines.next();
        for _ in 1..n {
            let coords: Vec<i32> = lines
                .next()
                .expect("error reading n")
                .split_ascii_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect();
            let x = coords[0];
            let y = coords[1];

            let x_pending_deviation = m - x;
            let y_pending_deviation = m - y;
            total_perimeter -= (x_pending_deviation * 2) + (y_pending_deviation * 2);
        }

        writeln!(out, "{total_perimeter}").expect("Error writing ans");
    }

    out.flush().ok();
}
