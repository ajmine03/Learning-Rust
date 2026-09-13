fn main(){
    let s = String::from("hell0 im ajmine");
    let (s,len) =  calclen(s);

    println!("The length of {s} is {len}");
}

fn calclen(s:String)->(String,usize){
    let result = s.len();
    (s,result)
}



