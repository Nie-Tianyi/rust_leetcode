struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let right_brackets = ['{', '[', '('];
        let left_brackets = ['}', ']', ')'];

        for v in s.chars() {
            if right_brackets.contains(&v) {
                stack.push(v);
            }
            if left_brackets.contains(&v) {
                let rb = stack.pop();
                if rb.is_none() {
                    return false;
                }
                if !is_matches(rb.unwrap(), v) {
                    return false;
                }
            }
        }

        stack.is_empty()
    }
}

fn is_matches(rb: char, lb: char) -> bool {
    if rb == '{' && lb == '}' {
        return true;
    }

    if rb == '[' && lb == ']' {
        return true;
    }

    if rb == '(' && lb == ')' {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert!(Solution::is_valid("{}".to_string()))
    }
}
