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
    let t: usize = read_numbers()[0];
    for _ in 0..t {
        let nums: Vec<u64> = read_numbers();
        let (row, col) = (nums[0], nums[1]);
        let result: u64;

        if row >= col {
            if row % 2 == 1 {
                result = (row - 1) * (row - 1) + col;
            } else {
                result = row * row - col + 1;
            }
        } else {
            if col % 2 == 1 {
                result = col * col - row + 1;
            } else {
                result = (col - 1) * (col - 1) + row;
            }
        }

        println!("{}", result);
    }
}