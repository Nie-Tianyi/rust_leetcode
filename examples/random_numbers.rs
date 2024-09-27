use std::io;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let size = lines
        .next()
        .unwrap_or(Ok("0".to_string()))
        .unwrap_or("0".to_string())
        .parse::<usize>()
        .unwrap_or(0);

    let mut inputs: Vec<i32> = Vec::with_capacity(size);

    for _ in 0..size {
        let num = lines
            .next()
            .unwrap_or(Ok("0".to_string()))
            .unwrap_or("0".to_string())
            .parse::<i32>()
            .unwrap_or(0);

        inputs.push(num);
    }

    inputs.sort();
    inputs.dedup();

    for i in inputs {
        println!("{i}")
    }
}
