fn main(){
    
    let days : [&str; 12] = ["first","second","third","fourth","fifth","sixth","seventh","eight","ninth","tenth","eleventh","twelfth"];

    let gifts : [&str; 12] = ["A partridge in a pear tree", "Two turtle doves","Three French hens", "Four calling birds", "Five gold rings","Six geese a-laying","Seven swans a-swimming","Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping","Twelve drummers drumming"];
    
    for i in 1..13{
        println!("On the {} day of Christmas\n my true love sent to me",days[i-1]);
        for j in 0..i{
             println!("{}",gifts[j]);
        }
        println!("\n");
    }
    println!("Happy Christmas!");
    
}
