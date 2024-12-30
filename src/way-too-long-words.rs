fn way_too_long_words() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.next();
    let out = &mut BufWriter::new(io::stdout());

    for __ in 0..n {
        let word: String = scanner.next();

        if word.len() <= 10 {
            writeln!(out, "{word}").ok();
        } else {
            let mut ans = String::new();
            let words = word.as_bytes();
            let words_len = words.len();

            ans.push(words[0] as char);
            ans.push_str(&(words_len - 2).to_string());
            ans.push(words[words_len - 1] as char);
            writeln!(out, "{ans}").ok();
        }
    }
}
