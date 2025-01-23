/// # 不同路径问题
/// 1. `dp[i][j]`表示机器人到点(i,j)有多少种走法，其中 0<=i,j<m,n
/// 2. 一个格子只有可能从它上面的一个格子向下走，或者右边的一个格子向左走。那么:
///     - `dp[i][j] = dp[i-1][j] + dp[i][j-1]`
/// 3. 机器人走到自己原点只有一种走法：`dp[0][0] = 1`
/// 4. 按照从左到右，从上到下的顺序遍历
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;

        let mut dp = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                if i == 0 || j == 0 {
                    dp[i][j] = 1;
                } else {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                }
            }
        }

        dp[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }
}
