fn main() {
    greet("ajmine");

    println!("{}", add(19, 1));
}


// Function with no return value
fn greet(name: &str) {
    println!("Hello {}", name);
}


// Function that returns a value
fn add(a: i32, b: i32) -> i32 {
    a + b
    // `a + b` is an expression, so its value is automatically returned.
    // No `return` keyword or semicolon is needed.
}