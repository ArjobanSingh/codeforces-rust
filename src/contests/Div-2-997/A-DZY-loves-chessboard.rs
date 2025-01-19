use std::io::{self, BufWriter, Read, Write};

const INVALID: char = '-';
const W: char = 'W';
const B: char = 'B';

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();
    let mut out = BufWriter::new(io::stdout().lock());

    let nums: Vec<usize> = lines
        .next()
        .expect("error reading n")
        .split_ascii_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

    let n = nums[0];
    let m = nums[1];

    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(n);

    for _ in 0..n {
        let row: Vec<char> = lines
            .next()
            .expect("error reading row") // Get the next line or panic
            .chars() // Split the line into words
            .collect();
        matrix.push(row);
    }

    for r in 0..n {
        let mut prev_cell = if r % 2 == 0 { W } else { B };
        for c in 0..m {
            let new_cell_val = if prev_cell == W { B } else { W };
            if matrix[r][c] != INVALID {
                matrix[r][c] = new_cell_val;
            }
            prev_cell = new_cell_val;
        }
    }

    for r in matrix {
        for c in r {
            write!(out, "{c}").expect("Error writing ans");
        }
        writeln!(out).expect("Error writing ans");
    }

    out.flush().ok();
}
