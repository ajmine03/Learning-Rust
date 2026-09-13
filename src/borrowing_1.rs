fn main() {
    let s1 = String::from("Hello");
    // s1 owns the String "Hello".

    let len = string_size(&s1);
    // `&s1` creates a reference to s1.
    // We are borrowing s1 instead of transferring its ownership.
    // So s1 can still be used after calling the function.

    println!("The size of '{}' is {}", s1, len);
}


// Function receives a reference to a String.
fn string_size(s: &String) -> usize {
    // `&String` means the function borrows the String.
    // `usize` is the type returned by `.len()`.

    s.len()
    // `.len()` returns the length of the String.
}