use std::cmp::PartialEq;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum Stage {
    Flag,
    Number,
    Done,
}
#[derive(Debug)]
#[allow(dead_code)]
enum Flag {
    Positive,
    Negative,
}

impl Solution {
    #[allow(dead_code)]
    pub fn my_atoi(s: String) -> i32 {
        let (res_vec, flag) = number_extractor(s);
        converter(res_vec, flag)
    }
}

fn converter(mut res_vec: Vec<i32>, flag: Flag) -> i32 {
    while res_vec.starts_with(&[0]) {
        res_vec.remove(0);
    }

    let mut res: i32 = 0;

    match flag {
        Flag::Positive => {
            for exp in 0..res_vec.len() {
                let n = res_vec.pop().expect("unexpected None in res_vec");
                let x = 10_i32.checked_pow(exp as u32);
                if x.is_none() {
                    return i32::MAX;
                }
                let y = n.checked_mul(x.unwrap());
                if y.is_none() {
                    return i32::MAX;
                }
                res = res.checked_add(y.unwrap()).unwrap_or(i32::MAX);
            }
        }
        Flag::Negative => {
            for exp in 0..res_vec.len() {
                let n = res_vec.pop().expect("unexpected None in res_vec");
                let x = 10_i32.checked_pow(exp as u32);
                if x.is_none() {
                    return i32::MIN;
                }
                let y = n.checked_mul(x.unwrap());
                if y.is_none() {
                    return i32::MIN;
                }
                res = res.checked_sub(y.unwrap()).unwrap_or(i32::MIN);
            }
        }
    }

    res
}

fn number_extractor(s: String) -> (Vec<i32>, Flag) {
    let mut res_vec: Vec<i32> = Vec::new();
    let mut flag = Flag::Positive;
    let mut stage = Stage::Flag;

    for c in s.chars() {
        match stage {
            Stage::Flag => match c {
                '-' => {
                    flag = Flag::Negative;
                    stage = Stage::Number;
                }
                '+' => {
                    flag = Flag::Positive;
                    stage = Stage::Number;
                }
                '0'..='9' => {
                    flag = Flag::Positive;
                    res_vec.push(match_char(c));
                    stage = Stage::Number;
                }
                ' ' => {} // 忽略
                _ => {
                    stage = Stage::Done;
                }
            },
            Stage::Number => match c {
                '0'..='9' => {
                    res_vec.push(match_char(c));
                }
                _ => stage = Stage::Done,
            },
            Stage::Done => {
                break;
            }
        }
    }

    (res_vec, flag)
}

#[inline]
fn match_char(c: char) -> i32 {
    match c {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        '0' => 0,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42)
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::my_atoi("   -042".to_string()), -42)
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::my_atoi("1337c0d3".to_string()), 1337)
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), i32::MIN)
    }

    #[test]
    fn test_5() {
        assert_eq!(
            Solution::my_atoi("  0000000000012345678".to_string()),
            12345678
        )
    }
}
