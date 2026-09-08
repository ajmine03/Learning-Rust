const AJMINE: u64 = 10_000;
// `const` declares a constant.
// A constant's value cannot be changed.
// Constants must have an explicit type.
// `10_000` is the same as `10000`; `_` is used for readability.

fn main() {
    let mut x = 32;
    // Variables are immutable by default in Rust.
    // `mut` makes the variable mutable, so its value can be changed.

    println!("x is equal to {}", x);
    // `{}` is a placeholder used to display a value.

    println!("The constant is equal to {}", AJMINE);

    x = 21;
    // Because `x` was declared with `mut`, we can change its value.

    println!("The updated x is equal to {}", x);
}