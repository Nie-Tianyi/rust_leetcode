use std::cmp::Ordering;

struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums;
        // 排序，按照从左到右数字大小排序
        nums.sort_by(|a, b| {
            unimplemented!()
        });

        nums.reverse();
        let nums: Vec<String> = nums.iter().map(|a| a.to_string()).collect();
        
        nums.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums = vec![10, 2];
        assert_eq!(Solution::largest_number(nums), String::from("210"));
    }

    #[test]
    fn test_2() {
        let nums = vec![3, 30, 34, 5, 9];
        assert_eq!(Solution::largest_number(nums), String::from("9534330"));
    }
    
    #[test]
    fn test_3() {
        let nums = vec![34323,3432];
        assert_eq!(Solution::largest_number(nums), String::from("343233432"));
    }
}
