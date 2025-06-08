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
    if n==2 || n==3 {
        println!("NO SOLUTION");
        return;
    } 

    for i in (2..=n).step_by(2) {
        print!("{} ",i);
    }

    for i in (1..=n).step_by(2) {
        print!("{} ",i);
    }
}
