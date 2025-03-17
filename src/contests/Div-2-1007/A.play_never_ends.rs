use std::io::{self, BufWriter, Read, Write};

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
        let n: i32 = lines
            .next()
            .expect("error reading line")
            .parse()
            .expect("error parsing n");

        if (n - 1) % 3 == 0 {
            writeln!(out, "YES").expect("Error writing empty line");
        } else {
            writeln!(out, "NO").expect("Error writing empty line");
        }
    }

    out.flush().ok();
}
