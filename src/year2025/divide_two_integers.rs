struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == 0 {
            return 0;
        }

        let mut counter = 0;
        let mut dvdnt_abs = dividend.unsigned_abs();
        let dvsr_abs = divisor.unsigned_abs();

        while dvdnt_abs >= dvsr_abs {
            counter += 1;
            dvdnt_abs -= dvsr_abs;
        }

        match (dividend > 0, divisor > 0) {
            (true, true) | (false, false) => {
                if counter == i32::MIN {
                    i32::MAX
                } else {
                    counter
                }
            }
            (true, false) | (false, true) => {
                if counter == i32::MAX {
                    i32::MIN
                } else {
                    -counter
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::divide(10, 3), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::divide(7, -3), -2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::divide(i32::MIN, -1), i32::MAX)
    }
}
