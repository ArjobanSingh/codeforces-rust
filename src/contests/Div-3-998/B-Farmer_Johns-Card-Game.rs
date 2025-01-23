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
        let arr: Vec<usize> = lines
            .next()
            .expect("Error going through n")
            .split_ascii_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let n = arr[0];

        let mut cows_begin_cards: Vec<(usize, i64)> = Vec::with_capacity(n);

        let mut is_n_distance = true;
        for idx in 0..n {
            if !is_n_distance {
                lines.next().expect("Error going through n");
                continue;
            }
            let mut cards: Vec<i64> = lines
                .next()
                .expect("Error going through n")
                .split_ascii_whitespace()
                .filter_map(|num| num.parse::<i64>().ok())
                .collect();

            cards.sort();

            // check if there's exactly m distance in every card.
            for i in 1..cards.len() {
                if cards[i] - cards[i - 1] != n as i64 {
                    is_n_distance = false;
                    break;
                }
            }

            cows_begin_cards.push((idx, cards[0]));
        }

        if !is_n_distance {
            writeln!(out, "-1").expect("Error writing empty line");
        } else {
            cows_begin_cards.sort_by(|a, b| a.1.cmp(&b.1));
            for it in cows_begin_cards {
                write!(out, "{} ", it.0 + 1).expect("Error writing empty line");
            }
            writeln!(out).expect("Error writing empty line");
        }
    }

    out.flush().ok();
}
