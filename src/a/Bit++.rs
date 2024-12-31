fn main() {
    let mut scanner = Scanner::new();
    let out = &mut BufWriter::new(io::stdout());
    let n: usize = scanner.next();

    let mut ans = 0;

    for _ in 0..n {
        let statement: String = scanner.next();
        match &statement[..] {
            "++X" | "X++" => ans += 1,
            "--X" | "X--" => ans -= 1,
            _ => (),
        }
    }
    writeln!(out, "{ans}").ok();
}
