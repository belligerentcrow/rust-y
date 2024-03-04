struct User {
    active : bool,
    username : String,
    email : String,
    sign_in_count : u64, 
}

fn main(){
    //instance of user
    let mut user1 = User {
        active = true, 
        username = String::from("belligerentcrow"),
        email = String::from("myemail@hehe.com"),
        sign_in_count = 1,
    };

    user1.email = String::from("my_new_email@hehe.com");
    

}
