struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = String::new();

        let strs: Vec<Vec<char>> = strs.iter().map(move |s| s.chars().collect()).collect();

        if let Some(first) = strs.first() {
            'a: for (ic, c) in first.iter().enumerate() {
                for s in &strs {
                    if s.get(ic) != Some(c) {
                        break 'a;
                    }
                }
                prefix.push(*c);
            }
        }

        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let ans = Solution::longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ]);
        assert_eq!(ans, "fl");
    }
}
