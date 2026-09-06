fn main(){
greet("ajmine");
println!("{}",add(19,1));
}  

//no return funtion
fn greet(name : &str){
    println!("hello {}",name)
}

//return type funtion
fn add(a:i32,b:i32)->i32{
    a + b // its an expression,its value automatically return value ; doesnt need semicolon
}

