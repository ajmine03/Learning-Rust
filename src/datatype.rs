fn main(){
    // datatypes;
    //scalars;
    let age : u32 = 18; // non-neg 32-bit integer
    let num : i32 = -1; // defult integer
    let temp : f64 = 32.12; // 64-bit flaot
    let boolean : bool = true; // true and false
    let grade: char = 'S'; // single charecter;
    let name = "Ajmine Adil"; // string;

    println!("age {}\n, temp {}\n, numb {}\n, status: {}\n, Result {}\n, name {}",
    age,temp,num,boolean,grade,name);

    //compound type:
    //tuple;
    let details: (&str, i32, char) = ("ajmine adil sadik",23,'s');
    println!("{} is {} years old and his fev char is {}",details.0,details.1,details.2);

    //array;
    let scores = [1,2,3,24,21];
    println!("2nd score is {}",scores[1]);


}  