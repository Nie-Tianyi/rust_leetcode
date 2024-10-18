struct Solution;

/// palindrome 回文字符串——从前读，从后读一样 e.g. bad
/// 找到给定字符串中最长的回文子字符串
impl Solution {
    // 广度优先暴力搜索 时间复杂度 O(n^3), 空间复杂度 O(1)
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }
        let chars = s.chars().collect::<Vec<char>>(); // &['b','b']
        for i in 0..chars.len() {
            // i=0,i=1
            let window_size = chars.len() - i; // window_size = 2
            for j in 0..=i {
                // j = 0
                if is_palindrome(&chars[j..j + window_size]) {
                    // [0..2]
                    return chars[j..j + window_size].iter().collect();
                }
            }
        }

        chars[0].to_string()
    }
}

fn is_palindrome(chars: &[char]) -> bool {
    let len = chars.len();
    for i in 0..len / 2 {
        if chars[i] != chars[len - i - 1] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert!(is_palindrome(&"bab".chars().collect::<Vec<char>>()));
        assert!(!is_palindrome(&"baac".chars().collect::<Vec<char>>()));
    }

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "bab".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::longest_palindrome("bb".to_string()),
            "bb".to_string()
        );
    }
}

struct Solution2;
// 扩散法
impl Solution2 {
    pub fn longest_palindrome(s: String) -> String {
        unimplemented!()
    }
}
