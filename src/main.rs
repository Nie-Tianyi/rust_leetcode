fn main() {
    let string:String = "abcabc".to_string();
    println!("{}", string.find("c").unwrap());
}