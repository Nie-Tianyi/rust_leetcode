fn main() {
    let mut my_string = String::from("Hello, world!");

    if let Some(first_char) = my_string.pop() {
        println!("Removed: {}", first_char);
    } else {
        println!("The string is empty.");
    }
}
