#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let v = merge_sorted_array(nums1, nums2);
        find_median(v)
    }
}

fn find_median(v: Vec<i32>) -> f64 {
    if v.len() % 2 == 0 {
        let i = v.len() / 2;
        (v[i - 1] + v[i]) as f64 / 2.0
    } else {
        let i = v.len() / 2;
        v[i] as f64
    }
}

fn merge_sorted_array(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
    let mut v1 = v1.into_iter();
    let mut v2 = v2.into_iter();

    let mut v = Vec::new();

    let mut number1 = v1.next();
    let mut number2 = v2.next();

    loop {
        match (number1, number2) {
            (None, None) => break,
            (Some(val), None) | (None, Some(val)) => {
                v.push(val);
                number1 = v1.next();
                number2 = v2.next();
            }
            (Some(val1), Some(val2)) => {
                if val1 <= val2 {
                    v.push(val1);
                    number1 = v1.next();
                } else {
                    v.push(val2);
                    number2 = v2.next();
                }
            }
        }
    }

    v
}
