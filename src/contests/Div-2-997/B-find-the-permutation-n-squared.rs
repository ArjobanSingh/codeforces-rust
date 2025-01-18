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
        let n: usize = lines
            .next()
            .expect("Error going through n")
            .parse()
            .unwrap();

        let mut perm = vec![0; n];

        for i in 1..=n {
            let mut before_items_count: usize = 0;
            let iter = lines
                .next()
                .expect("Error reading col")
                .as_bytes()
                .iter()
                .enumerate();

            for (j, &byte) in iter {
                let j = j + 1;

                // graph has no self loops, so ignore this
                if j == i {
                    continue;
                }

                let connected = (byte as char).to_digit(10);

                // j and i are the vertices in the Permutation array. Not the indices.
                // if j < i, the graph can only be connected, if j's index is also before i's index
                // else if j > i, the graph can only be connected, if j's index is after i's index.
                // Here for every i, we need to count how many items will be placed before it in the permutation.
                // So use above condition, and increase the count.
                if j < i {
                    if connected == Some(1) {
                        before_items_count += 1;
                    }
                } else {
                    if connected == Some(0) {
                        // Here not connected means, j's index is before i. because j node > i node.
                        before_items_count += 1;
                    }
                }
            }

            // after going through every adjancet position for this vertex i. we know how
            // many are placed before it in the permutation. So put this after all of them.
            // for 0 indexing. Just put at that before_items_count index.
            perm[before_items_count] = i;
        }

        for it in perm {
            write!(out, "{it} ").expect("Error writing ans");
        }

        writeln!(out).expect("Error writing empty line");
    }

    out.flush().ok();
}
