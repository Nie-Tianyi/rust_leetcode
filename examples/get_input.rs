use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    for line in lines {
        let line = line.expect("unexpected String");
        println!("{}",line);
    }
}