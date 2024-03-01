## gettin rusty with it

### intro

* rust indents with 4 spaces not a tab
* the ! after the function name = means you're calling a rust macro instead of a normal function. macros dont always follow the same rules as functions do
* rust compiles and outputs a binary exec. it's an ahead-of-time compiled language. 

### cargo

* cargo! it handles things like downloading libraries, dependencies, building the libs. 
* cargo expects the source files to live inside the src directory.
* use the ```$ cargo new``` command in the directory to make it a cargo 
* use the top level project directory ONLY for READMEs, license info, config files, and such. 
* use ```$ cargo build``` in the cargo directory to build the cargo, and ```$ cargo run``` to run it
* use the ```$ cargo check``` command to check the code to make sure it compiles but dont produce an executable. its faster 
 * the ```$ cargo build --release``` command is compiled with optimizations, but makes the compiling longer. This is when you've made the FINAL program you'll give to an user
* cargo provides the ```$ cargo update``` command to update dependencies but never goes higher than the number you put in the dependency header of Cargo.toml. you have to update that to use the next update of the dependency, otherwise Cargo.lock will make sure the dependency version YOU specified in Cargo.toml will be used 

### 2 - guessing game

* the standard library has a set of useful items described in the [standard library documentation](https://doc.rust-lang.org/stable/std/prelude/index.html) 
* create a variable with let
* VARIABLES are IMMUTABLE by default: to make a variable mutable you must add ```mut``` before the variable name 
    es: ``` let mut bananas = 5;```
* read_line returns a Result value, which is an enumeration which can have different states; each state is called a variant. The purpose of Result is to encode error-handling information. Result's variants are Ok and Err. 
* An instance of Result has an expect method that you can call; if Result has an Err value, the message you passed into expect is shown

* ```$ cargo doc --open``` builds documentation provided by all the dependencies locally and then opens it in the browser. based and poggers

* cmp --> Ordering type is an enum and has 3 variants: Less, Greater, and Equal.

* ```match``` and ```arms``` and ```patterns``` --> very powerful features of rust. the match is made of arms: you can handle each one of them this way. 

* Rust has a strong static type system, but it also has type inference. 

* Rust has mnay number types, and unless specified it defaults to i32. Rust cannot compare a string and a number, but we can trim and parse to "convert" !! ```let guess: u32 = guess.trim().parse().expect("Error");

* parse() returns a Result type just like read_line() does. 

* Rust lets you 'shadow' variables. This is often used for conversions from one type to another type. 

* ```loop{ }``` keyword, you exit it with a break; 

## 3 - Common programming concepts in Rust

* ```let mut``` makes a variable mutable
* ```const``` makes a variable constant -> instead of let. cant be mut

* shadowing works with different types, and goes back to the previous instance of the name when the shadow-ee's scope is over. the keyword ```let``` must be used when shadowing.
