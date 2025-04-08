struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i1, n) in nums.iter().enumerate() {
            let m = target - n;
            for (i2, x) in nums.iter().enumerate().skip(i1 + 1) {
                if *x == m {
                    return vec![i1 as i32, i2 as i32];
                }
            }
        }

        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn test_enum() {
        let v = ["a", "b", "c", "d"];

        for (i, item) in v.iter().enumerate().skip(1) {
            println!("索引: {}, 元素: {}", i, item);
        }
        // 索引: 1, 元素: b
        // 索引: 2, 元素: c
        // 索引: 3, 元素: d

        for (i, item) in v.iter().skip(1).enumerate() {
            println!("索引: {}, 元素: {}", i, item);
        }
        // 索引: 0, 元素: b
        // 索引: 1, 元素: c
        // 索引: 2, 元素: d
    }
}
