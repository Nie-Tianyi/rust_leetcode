use std::io;
use std::io::BufRead;

fn main() {
    // please define the Rust input here.
    let user_input = get_user_input();
    // please finish the function body here.
    let ans = find_ans(user_input.as_ref());
    // please define the Rust output here.
    print!("{}", ans);
}

fn find_ans(v: &[bool]) -> usize {
    if v.len() <= 7 {
        return match (v[1], v[2]) {
            (false, false) => 0,
            (true, false) | (false, true) => 1,
            (true, true) => 2,
        };
    }
    find_ans(&v[0..7]) + find_ans()
}

//给定定点，找到以其为定点的每一个子树
fn find_subtree(v: Vec<bool>, pivot: usize) -> Vec<bool> {
    let mut res = Vec::new();
    let i = pivot;
    while 2 * i + 1 <= v.len() {
        res.push(v[2 * i + 1]);
    }

    res
}

fn get_user_input() -> Vec<bool> {
    let mut res = Vec::new();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let line = lines.next().unwrap().unwrap();
    for c in line.split_ascii_whitespace() {
        if c == "0" {
            res.push(false);
        } else {
            res.push(true);
        }
    }

    res
}
