struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut prev_border_index = None;

        for i in 0..height.len() {
            if is_border(&height[..], i) {
                println!("find border {i}");
                match prev_border_index {
                    None => prev_border_index = Some(i),
                    Some(l) => {
                        res += bucket_volume(&height[l..=i]);
                        prev_border_index = Some(i);
                    }
                }
            }
        }

        res
    }
}

#[inline]
fn is_border(height: &[i32], index: usize) -> bool {
    // 如果是第一个，那么默认左边高度是零，那么只有当右边低于自己时，自己才是border
    if index == 0 && height[index + 1] < height[index] {
        return true;
    }
    // 如果是最后一个，那么右边默认高度是零，只有左边高度小于自己时，自己才是border
    if index == height.len() - 1 && height[index - 1] < height[index] {
        return true;
    }
    // 如果自己不是第一个，也不是最后一个，则当左右两边都小于等于自己，那么自己是border
    if index != 0
        && index != height.len() - 1
        && height[index - 1] <= height[index]
        && height[index] >= height[index + 1]
    {
        return true;
    }
    false
}
// 计算一个两边高，中间低的bucket的容积
#[inline]
fn bucket_volume(lh: &[i32]) -> i32 {
    let len = lh.len();
    let horizon = std::cmp::min(lh[0], lh[len - 1]);
    let res = lh
        .iter()
        .fold(0, |acc, &x| acc + std::cmp::max(0, horizon - x));
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_volume() {
        assert_eq!(bucket_volume(vec![2, 1, 2].as_ref()), 1);
        assert_eq!(bucket_volume(vec![2, 1, 0, 1, 3].as_ref()), 4);
    }

    #[test]
    fn test_solution() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
