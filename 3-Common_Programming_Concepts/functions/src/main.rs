fn main() {
    println!("Calling ... ");
    another_function(34);
}

fn another_function(x : i32){
    println!("I'm another function!");
    println!("Reporting the value... it's {x}!");
}
