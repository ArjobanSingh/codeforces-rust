use std::{
    collections::HashMap,
    io::{self, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();
    lines.next();

    let out = &mut BufWriter::new(io::stdout());
    let mut map: HashMap<&str, i32> = HashMap::new();

    for item in lines {
        if let Some(count) = map.get(item) {
            writeln!(out, "{item}{count}").ok();
        } else {
            writeln!(out, "OK").ok();
        }
        map.entry(item).and_modify(|p| *p += 1).or_insert(1);
    }
}
