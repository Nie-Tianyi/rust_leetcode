use std::io;
use std::io::BufRead;

/// 1. 汽水瓶
///     某商店规定：三个空汽水瓶可以换一瓶汽水，允许向老板借空汽水瓶（但是必须要归还）。
///     小张手上有n个空汽水瓶，她想知道自己最多可以喝到多少瓶汽水。
///     数据范围：输入的正整数满足: 1 ≤ n ≤ 100
///
/// 最后结果一定等于 n/2 向下取整
fn main() {
    let stdin = io::stdin();
    let mut res = Vec::new();

    // collect all the user inputs
    for line in stdin.lock().lines() {
        let ll = line.unwrap_or("0".to_string());
        let num = ll.parse::<i32>().unwrap_or(0);
        if num != 0 {
            res.push(num);
        } else {
            break;
        }
    }
    // count bottles
    let res: Vec<i32> = res.into_iter().map(count_bottles).collect();
    // print result
    for i in res {
        println!("{i}");
    }
}

fn count_bottles(num: i32) -> i32 {
    match num {
        0 | 1 => 0,
        2 => 1,
        n => n / 3 + count_bottles(n % 3 + n / 3),
    }
}
