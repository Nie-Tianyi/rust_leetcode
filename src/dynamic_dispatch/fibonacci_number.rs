/// 使用动态规划解决fibonacci数列问题，问题本身还可以使用很多其他方法例如递归解决，
/// 这里使用动态规划方法解决这个问题，用于练习使用动态规划问题解决问题。
///
/// # 斐波那契数列
/// 1. 数组含义为：`dp[i]` 代表了斐波那契数列中第i个数字
/// 2. 递推公式为：`dp[i] = dp[i-1] + dp[i-2]`
/// 3. 初始化： 数组前两个元素为1，即`dp[0] = 1`, `dp[1] = 1`
/// 4. 遍历顺序，从前往后
#[allow(dead_code)]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn fib(n: i32) -> i32 {
        // 特殊规定F(0) = 0
        if n == 0 {
            return 0;
        }
        let n = n as usize;
        // 初始化数组
        let mut dp = vec![0; n];

        for i in 0..n {
            if i == 0 || i == 1 {
                dp[i] = 1;
            } else {
                dp[i] = dp[i - 1] + dp[i - 2];
            }
        }

        dp[n - 1]
    }
}
