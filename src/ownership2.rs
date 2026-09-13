fn main() {

    // num একটি integer value
    let num = 20;
    // num কে add function এ পাঠানো হচ্ছে
    let result = add(num);
    // একটি String তৈরি করা হলো
    let name = String::from("ajmine adil");
    // name এর ownership ownership_function এ চলে যাবে
    ownership_function(name);
    println!("the {num} is and the result is {result}");
    // এখানে name আর ব্যবহার করা যাবে না
    // কারণ name এর ownership আগেই চলে গেছে
    // println!("The value of name is {name}");

    let s = give_ownership();
    println!("s = {s}");

    let s = takes_and_give(s);
    println!("s is {s}");
}

fn add(x: i32) -> i32 {
    x + 100
}

fn ownership_function(s: String) {
    // এখানে s এখন String এর owner
    println!("The name is {s}");
}

fn give_ownership() -> String {
    let s:String = String::from("This gives ownership from the funtion ");
    s
}

fn takes_and_give(s:String) -> String {
    println!("S is {s}");
    s
}