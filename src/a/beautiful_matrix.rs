fn main() {
    let mut scanner = Scanner::new();
    let out = &mut BufWriter::new(io::stdout());

    for r in 0i8..5 {
        for c in 0i8..5 {
            let num: i8 = scanner.next();
            if num == 1 {
                writeln!(out, "{}", r.abs_diff(2) + c.abs_diff(2)).ok();
                return;
            }
        }
    }
}
