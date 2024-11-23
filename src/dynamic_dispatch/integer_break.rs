/// # 整数拆分
///
/// 1. dp数组代表了当 n = i 时，所能达到的最大乘积
/// 2. 状态转移方程：`dp[i] = max(dp[i-1] * 1, dp[i-2] * 2,...,dp[1] * (i-1), (i-1) * 1, (i-2) * 2,..., 1 * (i-1))`
/// 3. 初始化： `dp[0]`和`dp[1]`没有意义，都初始化为 0
/// 4. 从 0 到 n 遍历

struct Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];

        for i in 2..=n {
            let mut max = 0;
            for j in 1..i {
                max = std::cmp::max(max, dp[i - j] * j);
                max = std::cmp::max(max, (i - j) * j);
            }
            dp[i] = max
        }

        println!("{dp:?}");

        dp[n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(Solution::integer_break(2), 1);
    }

    #[test]
    fn test_1() {
        assert_eq!(Solution::integer_break(10), 36);
    }
}
