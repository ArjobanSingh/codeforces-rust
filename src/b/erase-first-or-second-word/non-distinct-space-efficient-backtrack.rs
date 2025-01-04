use std::{
    collections::HashSet,
    io::{self, BufWriter, Read, Write},
};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Pos {
    start: usize,
    skip: usize,
}

fn backtrack(chunk: &str, pos: Pos, set: &mut HashSet<Pos>) {
    let Pos { start, skip } = pos;
    let chunk_size = chunk.len() - start - skip;

    // some condition?
    if chunk_size == 0 || set.contains(&pos) {
        return;
    }
    println!("Chunk {:?} and pos {:?}", chunk, pos);
    set.insert(pos);

    // Pick the first char and ignore next.
    backtrack(
        chunk,
        Pos {
            start,
            skip: skip + 1,
        },
        set,
    );

    // Remove the 1st (0th idx) char // start = skip + skip + 1
    if chunk_size > 1 {
        backtrack(
            chunk,
            Pos {
                start: start + skip + 1,
                skip: 0,
            },
            set,
        );
    }
}

fn backtrack_iter(chunk: String, set: &mut HashSet<String>) {
    let mut stack: Vec<String> = Vec::new();
    stack.push(chunk);

    while let Some(chunk) = stack.pop() {
        let chunk_size = chunk.len();
        if chunk_size == 0 || set.contains(&chunk) {
            continue;
        }
        set.insert(chunk.clone());

        // Remove the 2nd(1 idx) char
        let mut other = String::new();
        other.push_str(&chunk[..1]);
        if chunk_size > 2 {
            other.push_str(&chunk[2..]);
        }
        stack.push(other);

        // Remove the 1st (0th idx) char
        if chunk_size > 1 {
            stack.push(chunk[1..].to_string());
        }
    }
}

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

    for _ in 0..t {
        lines.next();
        let mut set: HashSet<Pos> = HashSet::new();
        let chunk = lines.next().expect("Error getting string").trim();
        backtrack(chunk, Pos { start: 0, skip: 0 }, &mut set);
        writeln!(out, "{}", set.len()).expect("Error writing");
    }

    out.flush().ok();
}
