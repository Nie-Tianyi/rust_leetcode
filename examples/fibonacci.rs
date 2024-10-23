fn main() {
    println!("{}", fibonacci(10))
}

fn fibonacci(n: i32) -> i32 {
    if n == 1 || n == 2 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
