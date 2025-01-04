use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();
    let t: usize = lines
        .next()
        .expect("error reading t")
        .trim()
        .parse()
        .expect("Error converting t");

    let mut out = BufWriter::new(io::stdout().lock());
    let mut cache = [0; 26];

    for _ in 0..t {
        lines.next();
        let chunk = lines.next().expect("Error getting string").trim();
        cache.fill(0); // reset array values in each iteration.

        let chunk_len = chunk.len();
        for (idx, &code) in chunk.as_bytes().iter().enumerate() {
            let cache_idx = code as usize - 97;
            if cache[cache_idx] > 0 {
                continue;
            }
            cache[cache_idx] = chunk_len - idx;
        }
        writeln!(out, "{}", cache.iter().fold(0, |acc, val| acc + val)).expect("Error writing");
    }

    out.flush().ok();
}
