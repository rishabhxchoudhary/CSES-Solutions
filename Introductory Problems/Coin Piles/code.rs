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
    let t: i64 = read_numbers()[0];
    for _ in 0..t {
        let line: Vec<i64> = read_numbers();
        let mut a = line[0];
        let mut b = line[1];
        if (a > 2*b) || (b>2*a) {
            println!("NO");
            continue;
        }
        a = a%3;
        b = b%3;
        if (a==1 && b==2) || (a==2 && b==1) || (a==0 && b==0) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}