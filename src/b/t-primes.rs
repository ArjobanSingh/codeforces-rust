use std::io::{self, BufWriter, Read, Write};

const MAX_LIMIT: u64 = 1_000_000_000_000;

fn sieve_of_eratosthenes() -> Vec<bool> {
    let limit = (MAX_LIMIT as f64).sqrt() as u64;
    let mut sieve = vec![true; (limit + 1) as usize];

    sieve[0] = true;
    sieve[1] = true;

    for i in 2..=(limit as f64).sqrt() as u64 {
        if sieve[i as usize] {
            for j in (i * i..=limit).step_by(i as usize) {
                sieve[j as usize] = false;
            }
        }
    }

    sieve
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();
    lines.next();

    let mut out = BufWriter::new(io::stdout().lock());
    let sieve = sieve_of_eratosthenes();

    for num in lines
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|it| it.parse::<f64>().ok())
    {
        let sqrt = num.sqrt();

        if num != 1.0 && sqrt.fract() == 0.0 && sieve[sqrt as usize] {
            writeln!(out, "YES").expect("Error in writing");
        } else {
            writeln!(out, "NO").expect("Error in writing");
        }
    }

    out.flush().ok();
}
