/// 练习动态规划 简单题 之一
///
/// # 解题思路
/// 1. `dp[i]` 代表了迈i级台阶可能的走法，例如：迈一级台阶只有一种走法（1），迈两级台阶有两种走法（11/2），
/// 迈三级台阶有三种走法（111/12/21），那么数组为`[0,1,2,3,...]`
/// 2. 递推公式，每一级台阶可能的走法只跟前两级台阶有关，例如第30级台阶只有可能是从29级台阶迈一步上来的或者从第28级
/// 台阶迈两步上来的。因此递推公式为`dp[i] = dp[i-1] + dp[i-2]`
/// 3. 由于迈一级台阶只有一种走法（1），迈两级台阶有两种走法（11/2），数组因该初始化为：`[0,1,2,...]`
/// 4. 应该从前往后遍历
struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;

        let mut dp = vec![0; n + 1];
        for i in 0..=n {
            if i <= 3 {
                dp[i] = i;
            } else {
                dp[i] = dp[i - 1] + dp[i - 2];
            }
        }

        dp[n] as i32
    }
}