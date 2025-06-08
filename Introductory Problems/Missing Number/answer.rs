use std::io::{BufRead, BufReader};

fn main() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let n: i64 = line.trim().parse().unwrap();

    line.clear();
    reader.read_line(&mut line).unwrap();
    let nums: Vec<i64> = line
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let expected_sum = n * (n + 1) / 2;
    let actual_sum: i64 = nums.iter().sum();

    // Print the missing number
    println!("{}", expected_sum - actual_sum);
}
