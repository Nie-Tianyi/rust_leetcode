use std::io;
use std::io::BufRead;

fn main() {
    let mut lines = io::stdin().lock().lines();
    let input = lines.next().unwrap().unwrap();
    let mut res = String::new();
    for ch in input.chars() {
        if !res.contains(ch) {
            res.push(ch);
        }
    }
    println!("{}", res)
}
