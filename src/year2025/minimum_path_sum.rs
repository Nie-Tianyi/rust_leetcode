struct Solution;

#[allow(unused)]
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        // 动态规划，初始化dp数组
        let m = grid.len();
        let n = match grid.first() {
            Some(n) => n.len(),
            None => panic!("invalid inputs"),
        };

        // dp[i][j] 表示走到 i，j所需要的最少的步数
        let mut dp = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                // 状态转移方程
                dp[i][j] = match (i, j) {
                    (0, 0) => grid[0][0],
                    (0, j) => dp[0][j - 1] + grid[0][j],
                    (i, 0) => dp[i - 1][0] + grid[i][0],
                    (i, j) => std::cmp::min(dp[i - 1][j], dp[i][j - 1]) + grid[i][j],
                }
            }
        }

        println!("{dp:?}");
        *dp.get(m - 1).unwrap().get(n - 1).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(Solution::min_path_sum(grid), 7);
    }

    #[test]
    fn test_2() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(Solution::min_path_sum(grid), 12)
    }
}
