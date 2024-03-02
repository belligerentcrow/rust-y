fn main() {
    let mut x1 = 5;
    
    let x = 2.0; //f64
    let y : f32 = 3.0; //f32

    //numeric operations
    let sum = 5+10;
    let difference = 20-5;
    let product = 5*3;
    let quotient = 30/2;
    let remainder = 10%3;
    println!("{sum},{difference},{product},{quotient}... it's all 15!");

    //bool
    let t = true;
    let f: bool = false;
    
    //chars
    let c = 'c';
    
    //tuples
    let tup : (i32,char,f64) = (17,'z',45.1); 
    let (a,b,c) = tup; 
    println!("my data is {a},{b},{c}");
    let ay = tup.0;
    let bee = tup.1;
    let see = tup.2;
    println!("my data is also {ay}, {bee}, {see}!");

    let fib = [1,1,2,3,5,8,13,21,34,55,89];
    let ten_nines = [9; 10];
    let first_fibo = fib[0];
    let fifth_fibo = fib[4];
}
