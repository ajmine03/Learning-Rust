fn main(){
    let mut s = String::from("Hello Ajmine!");
    let len = calclen(&mut s);

    println!("The size of {s} is {len}");
}

fn calclen(s : &mut String)->usize{
    s.push_str("end of function !");
    let result = s.len();
    result
}