fn main(){

    let my_string = String::from("hello world");

    let word = first_word(&my_string[..5]);
    println!("{}",word);   
    let word = first_word(&my_string[..]);
    println!("{}",word);   
    let word = first_word(&my_string);
    println!("{}",word);
    println!("{}",my_string);   

    //non-string slices
    let myarr = [1, 2, 3, 4, 5];
    let slice_of_it = &myarr[1..3];
    //this slice has the type &[i32] 
    assert_eq!(slice_of_it,&[2,3]);
}

fn first_word(s : &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
