struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut ids = vec![-1_i32; nums.len()]; // 3,2,2,3
        let mut count = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                ids[i] = count;
                count += 1;
            }
        } // -1,0,1,-1

        for i in 0..nums.len() {
            if ids[i] != -1 {
                nums.swap(ids[i] as usize, i);
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::year2025::remove_elements::Solution;

    fn do_test(mut v: Vec<i32>, val: i32, mut expected_v: Vec<i32>, expected_k: i32) {
        let k = Solution::remove_element(&mut v, val);
        assert_eq!(k, expected_k);

        let mut v = v[0..expected_k as usize].to_vec();
        v.sort();
        expected_v.sort();
        assert_eq!(v, expected_v);
    }

    #[test]
    fn test_1() {
        do_test(vec![3, 2, 2, 3], 3, vec![2, 2], 2)
    }

    #[test]
    fn test_2() {
        do_test(vec![0, 1, 2, 2, 3, 0, 4, 2], 2, vec![0, 1, 4, 0, 3], 5)
    }
}
