fn main() {
    let mut scanner = Scanner::new();
    let out = &mut BufWriter::new(io::stdout());

    let h: u64 = scanner.next();
    let w: u64 = scanner.next();
    let a: u64 = scanner.next();

    let ans_h = (h + a - 1) / a;
    let ans_w = (w + a - 1) / a;

    writeln!(out, "{}", ans_h * ans_w).ok();
}
