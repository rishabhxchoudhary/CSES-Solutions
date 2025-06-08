use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let mut split = line.trim().split_whitespace();
    let mut n: i32 = split.next().unwrap().parse().unwrap();

    while n != 1 {
        print!("{} ", n);
        if n & 1 != 0 {
            n = n * 3 + 1;
        } else {
            n = n / 2;
        }
    }
    println!("{}", n); 
}
