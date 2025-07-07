use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let hex = lines
        .next()
        .unwrap_or(Ok("0x0".to_string()))
        .unwrap_or("0x0".to_string());

    if !hex.starts_with("0x") {
        println!("invalid hexadecimal string");
    }

    let mut res = 0;
    let mut index = 0;

    let mut nums: Vec<char> = hex.chars().skip(2).collect();
    nums.reverse();
    for n in nums {
        let n = translate(n);
        if n.is_err() {
            continue;
        }
        res += n.unwrap() << (4 * index);
        index += 1;
    }

    println!("{}", res)
}

fn translate(c: char) -> Result<i32, &'static str> {
    match c {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        'a' | 'A' => Ok(10),
        'b' | 'B' => Ok(11),
        'c' | 'C' => Ok(12),
        'd' | 'D' => Ok(13),
        'e' | 'E' => Ok(14),
        'f' | 'F' => Ok(15),
        _ => Err("invalid char"),
    }
}
