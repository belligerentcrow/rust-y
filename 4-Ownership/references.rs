fn main(){
    //bad example without references
    let s1 = String::from("i love my wife");
    let (s2, len) = calculate_length_bad(s1);
    println!("The sentence is '{}' and its length is {}!", s2, len);
        
    //good example with references
    let s3 = String::from("I LOVE MY WIFE!!!");
    let len2 = calc_len_good(&s3);
    println!("The sentence is '{}' and its length is {}!!", s3, len2);
}

fn calculate_length_bad(mystring : String) -> (String, usize) {
    let length = mystring.len();
    (mystring, length)
} 

fn calc_len_good(s: &String) -> usize{
    s.len()
}
