use std::io::{self, BufRead};

fn read_numbers<T: std::str::FromStr>() -> Vec<T> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim()
        .split_whitespace()
        .map(|s| s.parse::<T>().ok().unwrap())
        .collect()
}

fn main() {
    let mut n: i64 = read_numbers()[0];
    let p = 5;
    let mut ans = 0;
    while n>0 {
        n = n / p;
        ans += n;
    }
    print!("{}", ans);
}