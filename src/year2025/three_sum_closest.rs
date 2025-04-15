struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut closest: i32 = nums[0..=2].iter().sum();
        let mut nums = nums;
        nums.sort();

        'outer: for first in 0..nums.len() - 2 {
            // 初始化三个指针分别只想第一个，第二个，和最后一个
            let mut left = first + 1;
            let mut right = nums.len() - 1;
            // 循环 第二个和最后一个指针
            'inner: while left < right {
                let sum = nums[first] + nums[left] + nums[right];
                match sum.cmp(&target) {
                    std::cmp::Ordering::Less => {
                        // 加起来比target小，第二个右移，看看有没有更大的组合
                        // 看看当前的sum是不是离目标更近
                        if (target - sum).abs() < (closest - target).abs() {
                            closest = sum;
                        }
                        left += 1;
                    }
                    std::cmp::Ordering::Equal => return sum,
                    std::cmp::Ordering::Greater => {
                        // sum比target大，最后一个指针左移
                        if (target - sum).abs() < (closest - target).abs() {
                            closest = sum;
                        }
                        right -= 1;
                    }
                }
            }
        }

        closest
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
    }
}
