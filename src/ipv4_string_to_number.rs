/// a robust algorithm that convert an IPv4 String to an u32 number

pub fn convert_ipv4_str_to_number(ip_addr: &str) -> Result<u32, &str> {
    // init an array that store result after split
    let mut array: Vec<u32> = Vec::new();
    // init a string that store intermediate result, e.g. "127"
    let mut inter_str = String::new();
    // init index of the array
    let mut index = 0_usize;
    // convert the ip address to an u32 array without "." e.g. [127_u32,0_u32,0_u32,1_u32]
    for character in ip_addr.chars() {
        match character {
            // if it matches a decimal number
            '0'..='9' => inter_str.push(character),
            // if it matches a "."
            '.' => {
                // convert string to number
                array.push(inter_str.parse().unwrap());
                // the number should belong to [0,255], otherwise it is invalid
                if array[index] > 255_u32 {
                    return Err("invalid IPv4 address");
                }
                // clear string
                inter_str.clear();
                // update index
                index += 1_usize;
            }
            // if it doesn't match any number, neglect it.
            _ => (),
        }
    }
    array.push(inter_str.parse().unwrap());
    // the number should belong to [0,255], otherwise it is invalid
    if array[index] > 255_u32 {
        return Err("invalid IPv4 address");
    }
    if array.len() != 4 {
        return Err("invalid IPv4 address");
    }
    // convert the array to an u32 Integer
    let res = (array[0] << 24) + (array[1] << 16) + (array[2] << 8) + array[3]; // the same as `array[3] << 0`
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_ip_to_number() {
        let res = convert_ipv4_str_to_number("192.168.1.1");
        assert_eq!(res.unwrap(), 3232235777_u32);
    }

    #[test]
    fn test_convert_ip_2_number_2() {
        let res = convert_ipv4_str_to_number("1.0.0.555");
        assert!(res.is_err());
    }

    #[test]
    fn test_convert_ip_to_number_3() {
        let res = convert_ipv4_str_to_number("1_9_2 . 1_6_8 . 1_ . 1_");
        assert_eq!(res.unwrap(), 3232235777_u32);
    }

    #[test]
    fn test_convert_ip_to_number_4() {
        let res = convert_ipv4_str_to_number("1.1.1.1.1");
        assert!(res.is_err());
    }

    #[test]
    fn test_convert_ip_to_number_5() {
        let res = convert_ipv4_str_to_number("1.1.1");
        assert!(res.is_err());
    }
}
