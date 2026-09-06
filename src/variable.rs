const AJMINE: u64 = 10_000; //this is constant declaretion;
fn main() {
    let mut x = 32; // x is immutable , variable cant replace unless use mut;
    println!("x is equal to {}",x); // {} is a placeholder;
    println!("the constant is equal to {}",AJMINE);

    x = 21;
    println!("the udated x is equal to {}",x);
    
}