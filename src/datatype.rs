fn main() {
    // =========================
    // Scalar Data Types
    // =========================

    let age: u32 = 18;
    // u32 = unsigned 32-bit integer.
    // It can store only non-negative numbers (0 and above).

    let num: i32 = -1;
    // i32 = signed 32-bit integer.
    // It can store both positive and negative numbers.
    // i32 is the default integer type in Rust.

    let temp: f64 = 32.12;
    // f64 = 64-bit floating-point number.
    // Used for numbers with decimal values.

    let boolean: bool = true;
    // bool can have only two values: true or false.

    let grade: char = 'S';
    // char stores a single Unicode character.
    // Character must use single quotes: 'S'

    let name = "Ajmine Adil";
    // String literal (&str).
    // Double quotes are used for string literals.


    println!(
        "age {}\ntemp {}\nnum {}\nstatus: {}\nresult {}\nname {}",
        age, temp, num, boolean, grade, name
    );


    // =========================
    // Compound Data Types
    // =========================

    // Tuple
    let details: (&str, i32, char) = ("Ajmine Adil Sadik", 23, 'S');

    // A tuple can store multiple values of different data types.
    // Access tuple values using their index: .0, .1, .2

    println!(
        "{} is {} years old and his favorite char is {}",
        details.0, details.1, details.2
    );


    // Array
    let scores = [1, 2, 3, 24, 21];

    // An array stores multiple values of the SAME data type.
    // Array indexing starts from 0:
    // scores[0] → 1
    // scores[1] → 2
    // scores[2] → 3

    println!("2nd score is {}", scores[1]);
}