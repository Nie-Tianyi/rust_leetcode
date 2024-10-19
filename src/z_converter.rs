struct Solution;

/// 将一个给定字符串 s 根据给定的行数 numRows ，以从上往下、从左到右进行 Z 字形排列。
///
/// 比如输入字符串为 "PAYPALISHIRING" 行数为 3 时，排列如下：
///
/// P   A   H   N
/// A P L S I I G
/// Y   I   R
/// 之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："PAHNAPLSIIGYIR"。
///
/// 请你实现这个将字符串进行指定行数变换的函数：
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 0 {
            panic!("invalid nums_rows");
        }
        if num_rows == 1 {
            return s;
        }
        let num_rows = num_rows as usize;
        let mut res: Vec<Vec<char>> = vec![vec![]; num_rows];
        let mut i = 0;
        let mut reverse_flag = false;
        for chr in s.chars() {
            res[i].push(chr);
            match reverse_flag {
                true => i -= 1,
                false => i += 1,
            }
            if i == num_rows - 1 {
                reverse_flag = true;
            }
            if i == 0 {
                reverse_flag = false;
            }
        }
        res.concat().iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR".to_string()
        )
    }
}
