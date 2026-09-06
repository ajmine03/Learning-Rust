// fn main(){
//     println!("hey ajmine!\nit's 5/9/26 !");
//     println!("Best of Luck\n");

// }


//Control Flow;
fn main(){
    let numb = 121;

    // if else statement
    if numb % 2 != 0 {  //doesnt need () in statement;
        println!("{} is odd",numb);
    }
    else{
        println!("{} is even",numb);
    }

    
    let lebel = if numb%2 == 0 {"even"} else{"odd"};
    println!("lebel is {}",lebel);

    //Loop
    let mut count  = 12;
    let result = loop {
        count = count * 2;
        if count > 100 {
            break count * 3;
        }

    };
    println!("\nResult = {} and loop is off\n",result);


    //While loop
    let mut n = 10;
    while n!=0 {
        println!("{}",n);
        n = n - 2;
    }
    println!("while is off\n");

    //in range;
    for i in 1..10{
        println!("i is {}",i);
    }
    println!("in range is off\n");

    //array in range;
    let scores = [10,20,30,40,50];
    for socr in scores {
        println!("score is {}",socr);
    }
    println!("in range array is off\n");



    
}
