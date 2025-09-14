struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums: Vec<String> = nums.iter().map(|a| a.to_string()).collect();
        // 排序，按照从左到右数字大小排序
        nums.sort_by(|a, b| (a.clone() + b).cmp(&(b.clone() + a)));

        nums.reverse();

        let mut res = nums.concat();

        trim_leading_zeros(&mut res);

        res
    }
}

fn trim_leading_zeros(s: &mut String) {
    while s.starts_with("0") && s.len() > 1 {
        s.drain(..1);
    }
}

#[cfg(test)]
mod tests {
    use super::{trim_leading_zeros, Solution};

    #[test]
    fn test_remove() {
        let mut s = String::from("0001");
        trim_leading_zeros(&mut s);
        println!("{s}")
    }

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
        let nums = vec![34323, 3432];
        assert_eq!(Solution::largest_number(nums), String::from("343234323"));
    }
}
