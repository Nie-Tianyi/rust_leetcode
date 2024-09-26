/// Given a string s, find the length of the longest substring without repeating characters.
///
/// LeetCode Link:[3.longest substring without repeating characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/description/)
///
/// **TODO**: cannot handle non-english characters. (because UTF-8 encoded)

pub fn length_of_longest_substring(s: &str) -> usize {
    // init a variable that store the global maximum
    let mut max_length = 0_usize;
    // init sliding window
    let mut substring: String = String::new();
    for character in s.chars() {
        match substring.find(character) {
            // if the sliding window does not contain this character
            // push the new character into the sliding window
            None => {
                substring.push(character);
            }
            // if the sliding window contains this character
            // update max_length
            // && delete all the characters before this character
            // && push new character into the substring
            Some(index) => {
                if max_length < substring.len() {
                    max_length = substring.len()
                };
                // remove the characters
                // todo: if the character is a chinese character, it would panic
                substring.drain(0..=index);
                substring.push(character);
            }
        }
    }
    // in case the string contains no repeating character
    if max_length < substring.len() {
        max_length = substring.len()
    };

    max_length
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        // the longest substring is "abc"
        let max_length = length_of_longest_substring("abcabcbb");
        assert_eq!(max_length, 3_usize);
    }
    #[test]
    fn test_2() {
        assert_eq!(length_of_longest_substring("bbbbb"), 1_usize);
    }
    #[test]
    fn test_3() {
        assert_eq!(length_of_longest_substring("pwwkew"), 3_usize)
    }
    #[test]
    fn test_4() {
        assert_eq!(length_of_longest_substring("aa b"), 3_usize)
    }
    #[test]
    #[should_panic(expected = "assertion failed: self.is_char_boundary(end)")]
    fn test_5() {
        assert_eq!(length_of_longest_substring("中文字符串中文"), 5_usize);
    }
}
