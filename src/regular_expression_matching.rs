struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut p = p.chars();
        for c in s.chars() {
            match p.next() {
                Some('.') => continue,
                Some('*') => break,
                Some(p) => {
                    if p != c {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
#[test]
fn test1() {
    assert!(!Solution::is_match("aa".to_string(), "a".to_string()));
}

#[cfg(test)]
#[test]
fn test2() {
    assert!(Solution::is_match("aa".to_string(), "a*".to_string()));
}

#[cfg(test)]
#[test]
fn test3() {
    assert!(Solution::is_match("ab".to_string(), ".*".to_string()));
}
