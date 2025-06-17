struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n >= 0 {
            quick_mul(x, n as u64)
        } else {
            1_f64 / quick_mul(x, (-n) as u64)
        }
    }
}

fn quick_mul(x: f64, n: u64) -> f64 {
    if n == 0 {
        return 1_f64;
    }

    let y = quick_mul(x, n / 2);

    if n % 2 == 1 {
        y * y * x
    } else {
        y * y
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_1() {
        assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::my_pow(2.0, -2), 0.25);
    }
}
