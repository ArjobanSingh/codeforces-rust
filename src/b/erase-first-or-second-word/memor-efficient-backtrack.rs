use std::{
    collections::HashSet,
    hash::Hash,
    io::{self, BufWriter, Read, Write},
};

#[derive(Debug, Eq, Copy, Clone)]
enum Chunk<'a> {
    Single(&'a str),
    Double((&'a str, &'a str)),
}

impl<'a> Chunk<'a> {
    fn len(&self) -> usize {
        match self {
            Chunk::Single(s) => s.len(),
            Chunk::Double((s1, s2)) => s1.len() + s2.len(),
        }
    }

    fn normalize(&self) -> String {
        match self {
            Chunk::Single(s) => s.to_string(),
            Chunk::Double((s1, s2)) => {
                let mut combined = String::new();
                combined.push_str(s1);
                combined.push_str(s2);
                combined
            }
        }
    }
}

impl<'a> PartialEq for Chunk<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.normalize() == other.normalize()
    }
}

impl<'a> Hash for Chunk<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.normalize().hash(state);
    }
}

fn backtrack<'a>(chunk: Chunk<'a>, set: &mut HashSet<Chunk<'a>>) {
    let chunk_size = chunk.len();
    if chunk_size == 0 || set.contains(&chunk) {
        return;
    }

    set.insert(chunk.clone());

    // Remove the 2nd(1 idx) char
    let mut new_chunk: (&str, &str) = ("", "");
    match chunk {
        Chunk::Single(s) => {
            new_chunk.0 = &s[..1];
            if chunk_size > 2 {
                new_chunk.1 = &s[2..]
            }
        }
        Chunk::Double((s1, s2)) => {
            new_chunk.0 = &s1[..1];
            if chunk_size > 2 {
                new_chunk.1 = &s2[1..]
            }
        }
    }
    backtrack(Chunk::Double(new_chunk), set);

    // Remove the 1st (0th idx) char
    if chunk_size > 1 {
        match chunk {
            Chunk::Single(s) => {
                backtrack(Chunk::Single(&s[1..]), set);
            }
            Chunk::Double((_, s2)) => {
                backtrack(Chunk::Single(s2), set);
            }
        }
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
        let mut set: HashSet<Chunk> = HashSet::new();
        let chunk = lines.next().expect("Error getting string").trim();
        backtrack(Chunk::Single(chunk), &mut set);
        writeln!(out, "{}", set.len()).expect("Error writing");
    }

    out.flush().ok();
}
