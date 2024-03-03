fn main(){

    let s1 = String::from("oh noooo");
    takes_ownership(s1);
    let s2 = String::from("hellllloo");
    let s3 = takes_and_gives_back(s2);
    let s4 = gives_ownership();
    println!("{s4} hath been received!");
}

fn takes_ownership(mys1: String){
    println!("{} is now mine", mys1);
}

fn takes_and_gives_back(mys2: String) -> String {
    mys2
}

fn gives_ownership() -> String{
    let mys3 = String::from("my sword");
    println!("I bestow {mys3} upon thee");
    mys3
}
