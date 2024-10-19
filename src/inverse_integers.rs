struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x.to_string();
        let flag = if x.starts_with("-") {
            Some(x.remove(0))
        } else {
            None
        };
        let mut x: String = x.chars().rev().collect();
        if let Some(f) = flag {
            x.insert(0, f);
        }
        x.parse().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::reverse(123), 321);
    }
}
