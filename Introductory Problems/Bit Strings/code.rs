use std::io::{self, BufRead};

fn read_numbers<T: std::str::FromStr>() -> Vec<T> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace()
        .map(|s| s.parse::<T>().ok().unwrap())
        .collect()
}

fn bin_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp /= 2;
    }

    result
}

fn main() {
    let n: u64 = read_numbers()[0];
    const MOD: u64 = 1_000_000_007;
    let answer = bin_pow(2, n, MOD);
    println!("{}", answer);
}