struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut indexes = vec![0; nums.len()]; // 1,1,1,1,2,2
        let mut count = 0;

        indexes[0] = count;
        for i in 1..nums.len() {
            if nums[i - 1] < nums[i] {
                count += 1;
            }
            indexes[i] = count;
        } // indexes = 0,0,0,0,1,1

        for i in 1..nums.len() {
            nums.swap(indexes[i], i);
        }

        *indexes.last().unwrap() as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn do_test(nums: &[i32], expected_nums: &[i32]) {
        let mut v = Vec::from(nums);
        let k = Solution::remove_duplicates(&mut v);
        let k = k as usize;
        assert_eq!(k, expected_nums.len());

        for i in 0..k {
            assert_eq!(v[i], expected_nums[i])
        }
    }

    #[test]
    fn test_1() {
        do_test(&[1, 1, 2], &[1, 2]);
    }

    #[test]
    fn test_2() {
        do_test(&[0, 0, 1, 1, 1, 2, 2, 3, 3, 4], &[0, 1, 2, 3, 4]);
    }
}
