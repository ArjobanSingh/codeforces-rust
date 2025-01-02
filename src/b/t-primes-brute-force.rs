use std::{
    io::{self, BufWriter, Read, Write},
    time::Instant,
};

fn main() {
    let test: i64 = 1000000000000;
    // let mut input = String::new();
    // io::stdin()
    //     .lock()
    //     .read_to_string(&mut input)
    //     .expect("Error reading line");

    // let mut lines = input.lines();
    println!("Here's the start: ");
    let mut out = BufWriter::new(io::stdout().lock());
    // lines.next();

    // for num in lines
    //     .next()
    //     .unwrap()
    //     .split(" ")
    //     .filter_map(|it| it.parse::<i64>().ok())
    let time = Instant::now();
    for idx in 0..100_000 {
        let num = test - idx;
        if num == 1 {
            writeln!(out, "NO").ok();
            continue;
        }

        let sqrt = (num as f64).sqrt();
        let mut count = 0;
        for i in 2..(sqrt.floor() as i64) {
            if num % i == 0 {
                count += 1;
                break;
            }
        }

        let is_perfect_sq = sqrt.fract() == 0.0;
        if count == 0 && is_perfect_sq {
            writeln!(out, "YES").expect("Error in writing");
        } else {
            writeln!(out, "NO").expect("Error in writing");
        }
    }
    let end = time.elapsed();

    out.flush().ok();
    let end_all = time.elapsed();
    println!("end of exe {:?}", end);
    println!("end of printing {:?}", end_all);
}
