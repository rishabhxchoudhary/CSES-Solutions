use std::io::{self, BufRead};

fn read_numbers<T: std::str::FromStr>() -> Vec<T> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace()
        .map(|s| s.parse::<T>().ok().unwrap())
        .collect()
}

fn main() {
    let _n: i64 = read_numbers()[0]; 
    let nums: Vec<i64> = read_numbers(); 

    let mut ans = 0;
    let mut max_so_far = nums[0];

    for &x in &nums[1..] {
        if x < max_so_far {
            ans += max_so_far - x;
        } else {
            max_so_far = x;
        }
    }
    println!("{}", ans)
}