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
    let total = n * (n + 1) / 2;

    if total & 1 == 1 {
        println!("NO");
        return;
    }

    let target = total / 2;
    let mut set1 = Vec::with_capacity(n as usize);
    let mut set2 = Vec::with_capacity(n as usize);
    let mut s1 = 0;

    for i in (1..=n).rev() {
        if s1 + i <= target {
            set1.push(i);
            s1 += i;
        } else {
            set2.push(i);
        }
    }

    println!("YES");
    println!("{}", set1.len());
    for num in &set1 {
        print!("{} ", num);
    }
    println!();
    println!("{}", set2.len());
    for num in &set2 {
        print!("{} ", num);
    }
    println!();

}