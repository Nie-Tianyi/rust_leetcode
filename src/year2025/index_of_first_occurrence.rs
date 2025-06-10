struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() > haystack.len() || haystack.is_empty() || needle.is_empty() {
            return -1;
        }
        
        let haystack = haystack.into_bytes();
        let needle = needle.into_bytes();
        
        for i in 0..haystack.len() {
            if haystack[i] == needle[0] {
                let needle_len = needle.len();
                if i + needle_len > haystack.len() {
                    break
                }
                
                if Self::is_identical(&haystack[i..i + needle_len], &needle) {
                    return i as i32
                }
            }
        }

        -1
    }

    #[inline(always)]
    fn is_identical(s1: &[u8], s2: &[u8]) -> bool {
        if s1.len() != s2.len() {
            return false
        }
        
        for (a, b) in s1.iter().zip(s2) {
            if a != b {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
            0
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::str_str("leetcode".to_string(), "leeto".to_string()),
            -1
        );
    }
}
