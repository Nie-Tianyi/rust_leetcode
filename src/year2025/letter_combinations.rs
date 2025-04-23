struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut res = Vec::new();
        
        for c in digits.chars() {
            let v = letter_mapping(c);
            res = multiply(res.as_slice(), v.as_slice())
        }
        
        res
    }
}

fn multiply(v1: &[String], v2: &[String]) -> Vec<String> {
    let mut res = Vec::new();
    
    if v1.is_empty() {
        return v2.to_vec();
    }
    
    for s1 in v1 {
        for s2 in v2 {
            res.push(format!("{s1}{s2}"))
        }
    }
    
    res
}

fn letter_mapping(c: char) -> Vec<String> {
    match c {
        '2' => vec![String::from('a'), String::from('b'), String::from('c')],
        '3' => vec![String::from('d'), String::from('e'), String::from('f')],
        '4' => vec![String::from('g'), String::from('h'), String::from('i')],
        '5' => vec![String::from('j'), String::from('k'), String::from('l')],
        '6' => vec![String::from('m'), String::from('n'), String::from('o')],
        '7' => vec![String::from('p'), String::from('q'), String::from('r'), String::from('s')],
        '8' => vec![String::from('t'), String::from('u'), String::from('v')],
        '9' => vec![String::from('w'), String::from('x'), String::from('y'), String::from('z')],
        _ => panic!("unknown char"),
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        )
    }
}
