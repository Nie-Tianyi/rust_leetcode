use std::io;
use std::io::BufRead;
/// 有一个数组 a[N] 顺序存放 0 ~ N-1 ，要求每隔两个数删掉一个数，
/// 到末尾时循环至开头继续进行，求最后一个被删掉的数的原始下标位置。
/// 以 8 个数 (N=7) 为例 :｛ 0，1，2，3，4，5，6，7 ｝，0 -> 1 -> 2 (删除)
/// -> 3 -> 4 -> 5 (删除) -> 6 -> 7 -> 0 (删除),如此循环直到
/// 最后一个数被删除。
///
// 3 6
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let num = lines
        .next()
        .unwrap_or(Ok("0".to_string()))
        .unwrap_or("0".to_string())
        .parse::<usize>()
        .unwrap_or(0);

    let num = find_last(num);

    println!("{}", num)
}

fn find_last(n: usize) -> usize {
    if n <= 1 {
        return 0;
    }
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push(i);
    }
    pop_elements(&mut v)
}

// 3,6
fn pop_elements(v: &mut Vec<usize>) -> usize {
    let mut index = 0;
    let mut res = 0;
    loop {
        if v.len() <= 2 {
            return res;
        }
        if (index + 1) % 3 == 0 {
            res = v.remove(index % v.len());
        }
        index += 1;
    }
}
