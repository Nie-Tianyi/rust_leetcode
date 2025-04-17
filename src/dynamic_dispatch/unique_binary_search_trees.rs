/// # 不同的二叉搜索树
/// 1. `dp[i]` 表示在 1..i 可以构成多少种子树，例如 `dp[1] = 1 dp[2] = 2 dp[3] = 5`
/// 2. 1..n 可以构建 以 1 为顶点， 0..0 颗左子树 * 2..n 颗右子树，以2为顶点， 1..1 颗 右子树 * 3..n颗右子树
///    同时，1..3 可以构建的树的数量等于 2..4，也等于 3..5。
///    因此 `dp[i] = dp[0] * dp[i-1] + dp[1] * dp[i-2] + ... + dp[i-1] * dp[0]`。
/// 3. 初始化 `dp[0] = 1` `dp[1] = 1`
/// 4. 从 0..=n遍历
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];

        dp[0] = 1;
        dp[1] = 1;

        for i in 2..=n {
            for j in 0..=i - 1 {
                dp[i] += dp[j] * dp[i - 1 - j];
            }
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::num_trees(1), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::num_trees(3), 5);
    }
}
