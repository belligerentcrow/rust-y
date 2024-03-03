fn main(){
    //push - mutable
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}",s);

    //error of ownership!
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}", s1);

    //clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}! {}!", s1, s2);
   
    //Stock-Only data copy example: this is valid 
    let x1 = 5;
    let x2 = x1;
    println!("{}", x1);

    //ownership usage example 
    let s3 = String::from("my_string_3");
    takes_ownership(s3);
    
    //using s3 here would cause a compile-time error

    let x3 = 6;
    makes_copy(x3);
    

    

}

fn takes_ownership(a_string: String){
    println!("{} was used here", a_string);
}

fn makes_copy(inttt : i32){
    println!("{} was used here", inttt);    
}
