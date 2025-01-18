use std::{
    cmp::Ordering,
    io::{self, BufWriter, Read, Write},
};

// If we Ignore reading cost of Input which is (N^2)
// The actual Algorithm is only (N * lgN). Sorting in certain way.
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
        let n: usize = lines
            .next()
            .expect("Error going through n")
            .parse()
            .unwrap();

        let mut perm = Vec::with_capacity(n);
        let mut matrix: Vec<String> = Vec::with_capacity(n);

        for i in 1..=n {
            perm.push(i);
            matrix.push(lines.next().expect("error reading line").to_string())
        }

        perm.sort_by(|&a, &b| {
            // Perm array is 1-n. Convert a and b to 0 base index.
            let is_connected = (matrix[a - 1].as_bytes()[b - 1] as char).to_digit(10) == Some(1);

            // if is_connected, put the smaller vertex out of a and b, before the other vertex.
            // else if not connected, put the larger vertex out of a and b, before the other vertex.
            if is_connected {
                if a < b {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            } else {
                if a < b {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        });

        for it in perm {
            write!(out, "{it} ").expect("Error writing ans");
        }

        writeln!(out).expect("Error writing empty line");
    }

    out.flush().ok();
}
