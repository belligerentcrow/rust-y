fn main(){
    let mut s = String::from("Hello");
    change(&mut s);
    
    {   let ref1 = &mut s;
        ref1.push_str("!!");  }
    //since ref1 is out of scope, we can make another mutable reference 
    let ref2 = &mut s;
    println!("{}", ref2);
    
    let mut s2 = String::from("hi");
    let r1 = &s2;
    let r2 = &s2;
    println!("{} {}", r1, r2);
    //this works because i wont use r1 nor r2 after this point
    let r3 = &mut s2;


}

fn change(mystring : &mut String) {
    mystring.push_str(" World!");
}
