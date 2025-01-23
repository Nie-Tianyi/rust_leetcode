use std::collections::HashMap;

#[allow(dead_code)]
pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let mut res = 0_i64;

    let (nums1, max) = frequency_and_max(nums1);
    let (nums2, _) = frequency_and_max(nums2);

    for (key2, value2) in &nums2 {
        for i in (key2 * k..=max).step_by((key2 * k) as usize) {
            if let Some(&value1) = nums1.get(&i) {
                res += value1 as i64 * (*value2) as i64;
            }
        }
    }

    res
}

#[inline(always)]
fn frequency_and_max(v: Vec<i32>) -> (HashMap<i32, i32>, i32) {
    let mut max = 0;
    let mut res = HashMap::<i32, i32>::new();
    for i in v {
        res.entry(i).and_modify(|c| *c += 1).or_insert(1);
        max = std::cmp::max(max, i);
    }
    (res, max)
}

#[cfg(test)]
mod tests {
    use super::number_of_pairs;

    #[test]
    fn test() {
        let nums1 = vec![1, 3, 4];
        let nums2 = vec![1, 3, 4];

        assert_eq!(number_of_pairs(nums1, nums2, 1), 5);
    }
}
