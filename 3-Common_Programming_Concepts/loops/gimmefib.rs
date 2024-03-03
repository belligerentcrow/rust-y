use std::io;

fn main(){
    
    let mut fibonumb = String::new();
        io::stdin()
            .read_line(&mut fibonumb)
            .expect("Didn't work!");
        let mynumber : i32 = fibonumb.trim().parse().expect("not working");
        calc_fib(mynumber);
    
}

fn calc_fib(n : i32) -> i32{
    
    if n<0 {return 0}
    else {
        match n {
            1 | 2 => {println!("1");
                    return 1},
            3 => {println!("2");
                    return 2},
            _ => { println!("{} and {}",n-1,n-2);
                 return calc_fib(n-1) + calc_fib(n-2) } 
        }
    }   
}
