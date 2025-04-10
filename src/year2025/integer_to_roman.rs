struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn int_to_roman(num: i32) -> String {
        match num {
            1 => "I".to_string(),
            2..4 => String::from("I") + Self::int_to_roman(num - 1).as_str(),
            4 => "IV".to_string(),
            5 => "V".to_string(),
            6..9 => String::from("V") + Self::int_to_roman(num - 5).as_str(),
            9 => "IX".to_string(),
            10 => "X".to_string(),
            11..40 => String::from("X") + Self::int_to_roman(num - 10).as_str(),
            40 => "XL".to_string(),
            41..50 => String::from("XL") + Self::int_to_roman(num - 40).as_str(),
            50 => "L".to_string(),
            51..90 => String::from("L") + Self::int_to_roman(num - 50).as_str(),
            90 => "XC".to_string(),
            91..100 => String::from("XC") + Self::int_to_roman(num - 90).as_str(),
            100 => "C".to_string(),
            101..400 => String::from("C") + Self::int_to_roman(num - 100).as_str(),
            400 => "CD".to_string(),
            401..500 => String::from("CD") + Self::int_to_roman(num - 400).as_str(),
            500 => "D".to_string(),
            501..900 => String::from("D") + Self::int_to_roman(num - 500).as_str(),
            900 => "CM".to_string(),
            901..1000 => String::from("CM") + Self::int_to_roman(num - 900).as_str(),
            1000 => "M".to_string(),
            1001..4000 => String::from("M") + Self::int_to_roman(num - 1000).as_str(),
            _ => panic!("unidentified number"),
        }
    }
}
