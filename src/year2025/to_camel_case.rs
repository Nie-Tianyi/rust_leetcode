fn to_camel_case(text: &str) -> String {
    let mut start_of_a_word = false;
    let mut res = String::new();

    for c in text.chars() {
        match c {
            '-' | '_' => {
                start_of_a_word = true;
            }
            'a'..='z' | 'A'..='Z' => {
                if start_of_a_word {
                    res.push(c.to_ascii_uppercase());
                } else {
                    res.push(c);
                }
                start_of_a_word = false;
            }
            _ => {
                res.push(c);
                start_of_a_word = true;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::to_camel_case;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn do_test(s: &str, expected: &str) {
        assert_eq!(to_camel_case(s), expected, "{ERR_MSG} with text = \"{s}\"")
    }

    #[test]
    fn fixed_tests() {
        do_test("", "");
        do_test("the_stealth_warrior", "theStealthWarrior");
        do_test("The-Stealth-Warrior", "TheStealthWarrior");
        do_test("A-B-C", "ABC");
        do_test("this is a test", "this Is A Test");
        do_test("这个是测试 test", "这个是测试 Test");
    }
}
