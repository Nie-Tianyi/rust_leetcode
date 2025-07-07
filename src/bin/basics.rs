fn main() {
    let str1 = ['a', 'b', 'c'];
    let str2 = ['a', 'b', 'c'];

    static STR3: [char; 3] = ['a', 'b', 'c'];
    static STR4: [char; 3] = ['a', 'b', 'c'];

    let str5 = Box::new(['a', 'b', 'c']);
    let str6 = Box::new(['a', 'b', 'c']);

    println!("{}", str1 == str2); // true
    println!("{}", STR3 == STR4); // true
    println!("{}", str5 == str6); // true

    println!("{}", str1 == STR3); // true
                                  // println!("{}", str1 == str5); // error, type mismatch
}
