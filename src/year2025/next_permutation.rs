struct Solution;

#[allow(unused)]
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let last = nums.len() - 1;
        for i in 1..nums.len() {
            if !is_descending_order(&nums[last - i..=last]) {
                // 挑一个稍微比 nums[last - i] 大一点的数，放到最前面，后面的升序排列
                let mut slice = Vec::new();
                nums[last - i..=last].clone_into(&mut slice);
                slice.iter_mut().for_each(|a| *a -= nums[last - i]);
                // 找到大于零且最小的数字的index
                let mut min_index = 0;
                for j in 0..slice.len() {
                    if slice[j] > 0 && min_index == 0 {
                        min_index = j;
                    }
                    if slice[j] > 0 && slice[j] < slice[min_index] {
                        min_index = j;
                    }
                }
                nums.swap(last - i + min_index, last - i);
                nums[last - i + 1..=last].sort();
                return;
            }
        }
        // 如果所数字已经是最大的了，返回最小值
        nums.reverse();
    }
}

// 判断一个数组是否从大到小排列
fn is_descending_order(s: &[i32]) -> bool {
    for i in 0..s.len() {
        if i + 1 < s.len() && s[i] < s[i + 1] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let mut nums = vec![2, 3, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![3, 1, 2]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_3() {
        let mut nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);
    }
}
