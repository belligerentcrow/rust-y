use std::io;

fn main() {
    println!("Calling ... ");
    another_function(34);
    let mut input = String::new();
    io::stdin().read_line(&mut input); 
    let stringy = return_string(input);
    println!("{stringy}");   
}

fn another_function(x : i32){
    println!("I'm another function!");
    println!("Reporting the value... it's {x}!");
}


fn return_string(my_input : std::string::String) -> std::string::String{
    if my_input.trim() == "activate"{
            "protocol activated".to_string()
        } else {
            "protocol dormant".to_string()
        }
    }
