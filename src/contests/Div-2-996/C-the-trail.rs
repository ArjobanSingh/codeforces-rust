use std::io::{self, BufWriter, Read, Write};

// We know that the sum of grid is
// Sum of each row * no. of rows or
// Sumf of each col * no. of cols.
// Sum of each row and col also match each other.
// So formuale is N*X = M*X = Total sum of grid
// where N is no. of rows and M is no. of cols and
// x is sum of that row or col.
// As N and M can be different, so X can only b 0.
const X: i64 = 0;
const D: char = 'D';

fn get_cell_idx(r: usize, c: usize, cols: usize) -> usize {
    r * cols + c
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
        let dimens: Vec<usize> = lines
            .next()
            .expect("Error reading line")
            .split_ascii_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let n = dimens[0];
        let m = dimens[1];

        let path: Vec<char> = lines
            .next()
            .expect("Error reading path")
            .to_string()
            .chars()
            .collect();

        // Keep track of sums of each row and col. Intially we'll have partial sums
        // Update them when handling the main move logic.
        let mut rows_sum: Vec<i64> = vec![0; n];
        let mut cols_sum: Vec<i64> = vec![0; m];
        let mut matrix: Vec<i64> = Vec::with_capacity(n * m);

        for row_idx in 0..n {
            for (col_idx, cell) in lines
                .next()
                .expect("Error reading line")
                .split_ascii_whitespace()
                .enumerate()
            {
                let cell: i64 = cell.parse().expect("Error parsing cell");
                rows_sum[row_idx as usize] += cell;
                cols_sum[col_idx] += cell;
                matrix.push(cell);
            }
        }

        let mut r: usize = 0;
        let mut c: usize = 0;
        let mut dir_idx = 0;

        loop {
            // if out of bounds, checking row is enough as per our move logic
            if r == n {
                break;
            }

            let &dir = path.get(dir_idx).unwrap_or(&D);

            if dir == D {
                // next dir is down, so get this cell value by subtracing
                // this row sum from 0
                let cell = X - rows_sum[r];
                matrix[get_cell_idx(r, c, m)] = cell;

                // Update the corresponding column and row sum
                rows_sum[r] += cell;
                cols_sum[c] += cell;
                r += 1;
            } else {
                // next dir is Right, so get this cell value by subtracing
                // this col sum from 0
                let cell = X - cols_sum[c];
                matrix[get_cell_idx(r, c, m)] = cell;
                rows_sum[r] += cell;
                cols_sum[c] += cell;
                c += 1;
            }
            dir_idx += 1;
        }

        for r in 0..n {
            for c in 0..m {
                write!(out, "{} ", matrix[get_cell_idx(r, c, m)])
                    .expect("Error writing Empty line");
            }
            writeln!(out, "").expect("Error writing Empty line");
        }
    }
    out.flush().ok();
}
