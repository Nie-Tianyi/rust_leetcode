struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        match subsearch(&nums[..], target) {
            Some(index) => index as i32,
            None => -1,
        }
    }
}

fn subsearch(nums: &[i32], target: i32) -> Option<usize> {
    if nums.len() == 1 && nums[0] == target {
        return Some(0);
    }

    if nums.len() == 1 && nums[0] != target {
        return None;
    }

    let first = 0;
    let mid = nums.len() / 2;
    let last = nums.len() - 1;

    if nums[first] <= nums[mid] {
        if nums[last] <= target && target <= nums[mid] {
            return subsearch(&nums[0..=mid], target);
        }
        if nums[mid] < target || target <= nums[last] {
            let subindex = subsearch(&nums[mid + 1..=last], target);
            return subindex.map(|a| a + mid + 1);
        }
    }

    if nums[mid] <= nums[last] {
        if nums[mid] <= target && target <= nums[last] {
            let subindex = subsearch(&nums[mid + 1..=last], target);
            return subindex.map(|a| a + mid + 1);
        }
        if nums[first] <= target || target < nums[mid] {
            return subsearch(&nums[0..=mid], target);
        }
    }

    panic!("Undefined Situation")
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let ans = Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0);
        let expected = 4;
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_2() {
        let ans = Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3);
        let expected = -1;
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_3() {
        let ans = Solution::search(vec![1], 0);
        let expected = -1;
        assert_eq!(ans, expected);
    }
}
