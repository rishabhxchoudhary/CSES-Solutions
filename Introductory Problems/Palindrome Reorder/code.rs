use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let s = stdin.lock().lines().next().unwrap().unwrap();
    let s = s.trim();

    // Frequency Map
    let mut count:HashMap<char, usize> = HashMap::new();
    for ch in s.chars() {
        *count.entry(ch).or_insert(0) += 1;
    }

    let chars: Vec<char> = count.keys().copied().collect();
    let mut half = String::new();
    let mut middle = String::new();

    for ch in chars {
        let cnt = count[&ch];
        if cnt % 2 == 1 {
            if !middle.is_empty() {
                println!("NO SOLUTION");
                return;
            }
            middle.push(ch);
        }
        half.push_str(&ch.to_string().repeat(cnt / 2));
    }

    let reversed_half: String = half.chars().rev().collect();
    println!("{}{}{}", half, middle, reversed_half);

}