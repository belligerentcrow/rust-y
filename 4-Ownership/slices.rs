fn main(){
    //bad solution
    let mut s = String::from("hello world!");
    let word = first_word(&s);
    
    //idea: slices!
    let mut s2 = String::from("Hello World!");
    let hello = &s2[0..5];  //0 can be dropped. [..5] works
    let world = &s2[6..12]; //12 can be dropped. [6..] works
    println!("{},{}", hello, world);

    //good solution
    
}

//bad!
fn first_word(s : &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()
}

fn first_word_good(s : &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
                return &s[0..i];
        }
    }
    &s[..]
}
