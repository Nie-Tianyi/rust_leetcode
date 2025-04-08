struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(x: i32) -> bool {
        let x = x.to_string();
        let x: Vec<char> = x.chars().collect();
        let len = x.len();
        if len % 2 == 0 {
            // 偶数个
            expansion(x, len / 2 - 1, len / 2)
        } else {
            // 奇数个
            expansion(x, len / 2, len / 2)
        }
    }
}

fn expansion(s: Vec<char>, li: usize, ri: usize) -> bool {
    if ri == li {
        for i in 1..=s.len() / 2 {
            if s.get(li - i) != s.get(ri + i) {
                return false;
            }
        }
    } else {
        for i in 0..s.len() / 2 {
            if s.get(li - i) != s.get(ri + i) {
                return false;
            }
        }
    }
    true
}
