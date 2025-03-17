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
        let n = lines
            .next()
            .expect("error reading line")
            .parse()
            .expect("error parsing n");

        let mut items: Vec<i32> = (1..=n).collect();
        let mut sum: f64 = 0.0;
        let mut i = 0;

        while i < items.len() {
            let num = items[i];
            sum += num as f64;
            let sqrt = sum.sqrt();

            if sqrt.fract() == 0.0 && i != items.len() - 1 {
                sum -= num as f64;
                items.swap(i, i + 1);
            } else {
                i += 1;
            }
        }

        if sum.sqrt().fract() == 0.0 {
            writeln!(out, "-1").expect("Error writing empty line");
        } else {
            items.iter().for_each(|it| {
                write!(out, "{} ", it).expect("Error writing empty line");
            });
            writeln!(out, "").expect("error writing empty");
        }
    }

    out.flush().ok();
}
