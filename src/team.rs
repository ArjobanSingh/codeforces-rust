fn team() {
    let mut scanner = Scanner::new();
    let out = &mut BufWriter::new(io::stdout());
    let n: usize = scanner.next();

    let mut ans = 0;
    for _ in 0..n {
        let mut sure_count = 0;
        for _ in 0..3 {
            if scanner.next::<u8>() == 1 {
                sure_count += 1;
            }
        }
        if sure_count > 1 {
            ans += 1;
        }
    }

    writeln!(out, "{ans}").ok();
}
