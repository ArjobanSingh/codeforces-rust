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
        let arr: Vec<char> = lines
            .next()
            .expect("Error going through n")
            .chars()
            .collect();

        let zeroes = arr
            .iter()
            .fold(0, |acc, &cur| if cur == '0' { acc + 1 } else { acc });

        writeln!(out, "{}", arr.len() - zeroes).expect("Error writing empty line");
    }

    out.flush().ok();
}
