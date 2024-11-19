/// # 使用最小花费爬楼梯
/// 1. `dp[i]` 数组的含义为爬到第i阶楼梯需要的最小花费
/// 2. 状态转移方程为：`dp[i] = min(dp[i-2] + cost[i-2], dp[i-1] + cost[i-1])`
/// 3. 初始化数组为 `[0,0,...]`, 因为可以选择从零或者一开始爬，因此`dp[0] = 0, dp[1] = 0`
/// 4. 遍历顺序，从前往后遍历
struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let len = cost.len() + 1;
        let top = cost.len();

        let mut dp = vec![0; len];
        for i in 0..len {
            if i == 0 || i == 1 {
                dp[i] = 0;
            } else {
                dp[i] = std::cmp::min(dp[i - 2] + cost[i - 2], dp[i - 1] + cost[i - 1])
            }
        }

        dp[top]
    }
}
