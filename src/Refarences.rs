fn main(){
    let s = String::from("Hello Ajmine!");
    let len = calclen(&s);

    println!("The size of {s} is {len}");
}

fn calclen(s : &String)->usize{
    let result = s.len();
    result
}