struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut dp = vec![vec![]; n + 1];
        for i in 0..=n {
            if i == 0 {
                dp[i] = vec!["".to_string()];
                continue;
            }
            if i == 1 {
                dp[i] = vec!["()".to_string()];
                continue;
            }

            let (left, right) = dp.split_at_mut(i);
            for j in 0..i {
                // combine j i-j
                let cmb = combine(&left[j], &left[i - j - 1]);
                right[0].extend_from_slice(&cmb);
            }
            dp[i].sort();
            dp[i].dedup();
        }

        std::mem::take(&mut dp[n])
    }
}

#[inline(always)]
fn combine(a: &[String], b: &[String]) -> Vec<String> {
    let mut res = Vec::new();
    for sa in a {
        for sb in b {
            res.push(format!("({sa}){sb}"))
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn do_test(mut ans: Vec<String>, mut expected: Vec<&str>) {
        ans.sort();
        expected.sort();
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_1() {
        do_test(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"],
        )
    }

    #[test]
    fn test_2() {
        do_test(Solution::generate_parenthesis(1), vec!["()"]);
    }

    #[test]
    fn test_3() {
        do_test(Solution::generate_parenthesis(2), vec!["()()", "(())"]);
    }
}
