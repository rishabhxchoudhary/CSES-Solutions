use std::io::{BufRead, BufReader};

fn main() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let s = line.trim();

    if s.is_empty() {
        println!("{}", 0);
        return;
    }

    let mut chars = s.chars();
    let mut prev = chars.next().unwrap();
    let mut ans = 1;
    let mut final_ans = 1;

    for c in chars {
        if c == prev {
            ans += 1;
        } else {
            ans = 1;
            prev = c;
        }
        final_ans = final_ans.max(ans);
    }

    println!("{}",final_ans);
}
