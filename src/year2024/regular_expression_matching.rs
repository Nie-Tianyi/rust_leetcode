struct Solution;

impl Solution {
    #[allow(unreachable_code, dead_code)]
    pub fn is_match(s: String, p: String) -> bool {
        let patterns = Pattern::pattern_lexical_analysis(p);
        let mut chars = s.chars().peekable();
        let patterns = patterns.iter();

        for ptrn in patterns {
            // 按照模式挨个去匹配字符串
            match ptrn {
                Pattern::SingleChar(c) => {
                    let next_char = chars.next();
                    if next_char.is_none() {
                        return false;
                    }
                    if *c != next_char.unwrap() {
                        return false;
                    }
                }
                Pattern::SingleAny => {
                    let next = chars.next();
                    if next.is_none() {
                        return false;
                    }
                }
                Pattern::MultiAny => {
                    // 看后后面的字符串是否有能匹配得上的
                    todo!(); // 这里是有问题的
                    return true;
                }
                Pattern::MultiChar(c) => {
                    while chars.peek() == Some(c) {
                        chars.next();
                    }
                }
            }
        }
        // 如果所有模式都匹配完了，chars还有剩余，则返回false
        if chars.next().is_some() {
            return false;
        }

        true
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
enum Pattern {
    SingleChar(char), // 'c' 只能匹配一个字符c
    SingleAny,        // . 可以匹配任意单个字符
    MultiChar(char),  // "c*" 可以匹配零个或者任意多个字符 c
    MultiAny,         // .*  可以匹配任意字符串
}

impl Pattern {
    // 模版字符串语义分析, 解析字符串，返回一个数组的Pattern
    fn pattern_lexical_analysis(p: String) -> Vec<Pattern> {
        let mut patterns = Vec::new();
        let mut p = p.chars().peekable();

        while let Some(c) = p.next() {
            // 下一个字符如果是 *，则匹配 Multi*
            let flag = match p.peek() {
                Some('*') => true,
                Some(_) | None => false,
            };

            match c {
                '.' => {
                    if flag {
                        patterns.push(Pattern::MultiAny);
                    } else {
                        patterns.push(Pattern::SingleAny);
                    }
                }
                '*' => continue,
                c => {
                    if flag {
                        patterns.push(Pattern::MultiChar(c));
                    } else {
                        patterns.push(Pattern::SingleChar(c));
                    }
                }
            }
        }

        patterns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(!Solution::is_match("aa".to_string(), "a".to_string()));
    }

    #[test]
    fn test2() {
        assert!(Solution::is_match("aa".to_string(), "a*".to_string()));
    }

    #[test]
    fn test3() {
        assert!(Solution::is_match("ab".to_string(), ".*".to_string()));
    }

    #[test]
    fn test4() {
        assert!(Solution::is_match("aab".to_string(), "c*a*b".to_string()));
    }
}
