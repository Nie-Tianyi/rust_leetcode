struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn roman_to_int(s: String) -> i32 {
        fn matches_number(s: char) -> i32 {
            match s {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => unreachable!(),
            }
        }

        let numbers: Vec<i32> = s.chars().map(matches_number).collect();

        let mut res = 0_i32;

        for (index, &num) in numbers.iter().enumerate() {
            if index + 1 >= numbers.len() {
                res += num;
                continue;
            }

            if num < numbers[index + 1] {
                res -= num;
            } else {
                res += num;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let ans = Solution::roman_to_int("III".to_string());
        assert_eq!(ans, 3);
    }

    #[test]
    fn test_2() {
        let ans = Solution::roman_to_int("LVIII".to_string());
        assert_eq!(ans, 58);
    }

    #[test]
    fn test_3() {
        let ans = Solution::roman_to_int("MCMXCIV".to_string());
        assert_eq!(ans, 1994);
    }
}
