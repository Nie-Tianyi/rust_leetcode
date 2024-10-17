pub struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (_k, _is_biased) = find_k(nums1.len(), nums2.len());
        unimplemented!()
    }
}

// k is the position of the median number;
fn find_k(len1: usize, len2: usize) -> (usize, bool) {
    let len = len1 + len2;
    if (len1 + len2) % 2 == 0 {
        (len / 2 - 1, true) // median is at k and k + 1
    } else {
        (len / 2, false) // median is at k
    }
}
