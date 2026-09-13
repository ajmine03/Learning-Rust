fn main() {
    let s1 = String::from("Hello");
    // s1 owns the String "Hello". The String is created and stored on the heap.

    let s2 = s1.clone();
    // clone() creates a completely separate copy of the String.
    // Now both s1 and s2 own their own copy of "Hello".
    // We use clone() because assigning `s1` directly to `s2` would move the ownership
    // from s1 to s2, making s1 invalid.

    println!("\ns1 is = {} and s2 is = {}\n", s1, s2);
    
}