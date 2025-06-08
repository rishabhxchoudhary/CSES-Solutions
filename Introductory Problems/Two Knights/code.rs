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
    let n: i64 = read_numbers()[0];
    for i in 1..=n {
        let n2 = i*i;
        let total = n2 * (n2-1) / 2;
        let do_attack = 4 * (i-1) * (i-2);
        println!("{}", total - do_attack);
    }
}