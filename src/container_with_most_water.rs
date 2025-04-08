struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, height.len() - 1);
        let mut max_area = 0;

        let find_area = |l: usize, r: usize| std::cmp::min(height[l], height[r]) * ((r - l) as i32);

        while l < r {
            max_area = std::cmp::max(max_area, find_area(l, r));

            if height[l] <= height[r] {
                l += 1;
                continue;
            }

            if height[l] >= height[r] {
                r -= 1;
                continue;
            }
        }

        max_area
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }
}
