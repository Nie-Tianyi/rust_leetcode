/// # 不同路径二
///
/// 思路基本上跟不同路径一相同
/// 1. `dp[i][j]` 表示到第 (i,j) 格有多少种不同走法
/// 2. 状态转移方程： `dp[i][j] = dp[i-1][j] + dp[i][j-1]`, 其中 i!=0, j!=0, `obstacle_grid[i-1][j] != 1`,
///    `obstacle_grid[i][j-1] != 1`,
/// 3. 初始化：`dp[0][0] = 1`
/// 4. 从上到下，从左到右遍历
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[0][0] == 1 {
            return 0;
        }

        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();

        if obstacle_grid[m - 1][n - 1] == 1 {
            return 0;
        }

        let mut dp = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    dp[i][j] = 1;
                } else {
                    let up = if i == 0 || obstacle_grid[i - 1][j] == 1 {
                        0
                    } else {
                        dp[i - 1][j]
                    };
                    let left = if j == 0 || obstacle_grid[i][j - 1] == 1 {
                        0
                    } else {
                        dp[i][j - 1]
                    };
                    dp[i][j] = up + left;
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
    fn test() {
        let obstacle_grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), 2);
    }
}
