// =========================
// Control Flow
// =========================

fn main() {
    let numb = 121;


    // =========================
    // 1. If / Else Statement
    // =========================

    if numb % 2 != 0 {
        // Rust does not require parentheses around the condition.
        // `%` gives the remainder.
        // If the remainder is not 0, the number is odd.

        println!("{} is odd", numb);
    } else {
        println!("{} is even", numb);
    }


    // =========================
    // 2. If / Else as an Expression
    // =========================

    let label = if numb % 2 == 0 {
        "even"
    } else {
        "odd"
    };

    // In Rust, if/else can return a value.
    // Both branches must return the same type.
    // Here, both branches return a &str.

    println!("label is {}", label);


    // =========================
    // 3. Loop
    // =========================

    let mut count = 12;

    let result = loop {
        count = count * 2;

        if count > 100 {
            // `break` stops the loop.
            // `break value` can also return a value from the loop.
            break count * 3;
        }
    };

    println!("\nResult = {} and loop is off\n", result);


    // =========================
    // 4. While Loop
    // =========================

    let mut n = 10;

    while n != 0 {
        println!("{}", n);
        n = n - 2;
    }

    println!("while loop is off\n");


    // =========================
    // 5. For Loop with Range
    // =========================

    for i in 1..10 {
        // `1..10` means 1 through 9.
        // The upper limit (10) is NOT included.

        println!("i is {}", i);
    }

    println!("range loop is off\n");


    // =========================
    // 6. For Loop with Array
    // =========================

    let scores = [10, 20, 30, 40, 50];

    for score in scores {
        // The loop goes through each element of the array.
        // `score` contains one value at a time.

        println!("score is {}", score);
    }

    println!("array loop is off\n");
}