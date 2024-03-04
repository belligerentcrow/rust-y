struct User {
    active : bool,
    username : String,
    email : String,
    sign_in_count : u64, 
}

fn main(){
    //instance of user
    let mut user1 = User {
        active : true, 
        username : String::from("belligerentcrow"),
        email : String::from("myemail@hehe.com"),
        sign_in_count : 1,
    };
    user1.email = String::from("my_new_email@hehe.com"); 

    //using the Struct Update Syntax to create user2
    let user2 = User {
        active : user1.active,
        username : user1.username,
        email : String::from("second_user@hehe.com"),
        sign_in_count : user1.sign_in_count,       
    }
    
    //using the struct update syntax even more efficently
    let user3 = User {
        email : String::from("third_user@hehe.com"),
        ..user2
    }

    //BUT NOW! We can't use the ones that we've copied because they dont have the Copy behaviour.
    //They've been copied into the new struct! 
    //If we only copied the sign in and active then they would be valid because both u64 and bool
    //have the copy behaviour


    // at the Using Tuple Structs Without Named Fields chapter!!!!
    

}

fn build_user(email: String, username: String) -> User{
    User {
        active: true, 
        username: username,
        //quicker version (Field Init Shorthand):  
        //username, 
        email: email,
        //quicker version (Field Init Shorthand):
        //email,
        sign_in_count: 1,        
    }
}


