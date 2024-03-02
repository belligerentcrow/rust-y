fn main() {
    let mut count = 0;
    
    //loop
    'counting_up: loop {
        println!("Counting... {count}");
        let mut remaining = 10;
        loop {
            println!("Remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    
    //while
    println!("\n");
    let mut countdown = 10; 
    while countdown !=0 {
        println!("{countdown}!");
        countdown -= 1;
    }
    println!("LIFTOFF!");
    
    //array
    let myarr = [1,1,2,3,5,8,13,21];
    let mut index = 0;
    
    //array while
    while index <= 7 {
        println!("Fibonacci #{} = {}",index+1, myarr[index]);
        index += 1;
    }
    
    println!("\n Now with the for construct!\n");

    //array for
    for (i, el) in myarr.iter().enumerate() {
        println!("Fibonacci #{i} = {el}");
    }
    
    for numbb in (1..4).rev(){
        println!("{numbb}!");
    }
    println!("Liftoff!!");
    

}
