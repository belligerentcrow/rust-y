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

* Watch out for ```match``` statements!! they're like more powerful switches

* Rust has a strong static type system, but it also has type inference. 

* Rust has mnay number types, and unless specified it defaults to i32. Rust cannot compare a string and a number, but we can trim and parse to "convert" !! ```let guess: u32 = guess.trim().parse().expect("Error");

* parse() returns a Result type just like read_line() does. 

* Rust lets you 'shadow' variables. This is often used for conversions from one type to another type. 

* ```loop{ }``` keyword, you exit it with a break; 

## 3 - Common programming concepts in Rust

* ```let mut``` makes a variable mutable
* ```const``` makes a variable constant -> instead of let. cant be mut

* shadowing works with different types, and goes back to the previous instance of the name when the shadow-ee's scope is over. the keyword ```let``` must be used when shadowing.

### primitive types 
* there are 4 types of primary scalar types: **integers, floating-point, Booleans, characters**.
    * **integers** default to ```i32```
    * **floating points** default to ```f64``` (double precision)

* to specify a type use the notation: ```let x: i64 = 214;``` or ```let y: f32 = 2.0;```

* chars are specified with single quotes as opposed to strings. It uses UNICODE so it can represent more than ASCII: accented letters, chinese, japanese, korean characters, emojis, are characters. 

### compound types

* Tuple = ```let tup: (i32, f64, u8) = (500, 6.4, 1); 
    * they can contain different types. each position has a type. they are FIXED IN SIZE. 
* Tuples are single compounds, but it is possible to destructure them 
    * ```let (x,y,z) = tup;``` then print the value of y lends 6.4
    * you can also access an element by using the period: ```tup.0``` or ```tup.1```

* A tuple with no values is an "unit". It represents an empty value / return type. it's written ```()```

### Arrays 

* ```let fib = [1,1,2,3,5,8,13,21,34,55,89];```
* Elements inside an array must have the same type. Arrays also  have a FIXED size. 
    * VECTORS are able to grow and shrink in size.
* Use arrays when you're sure the number of them won't change.
    * ex: array of 5 i32 elements = ```let a: [i32; 5] = [1,2,3,4,5];```
    * array of 5 threes ```let a = [3; 5]; 

### functions

* they work very similarly as c++
* you MUST declare the type of each function parameter.

## Statements vs Expressions

* A **Statement** is an instruction that performs an/some action/s and does not return a value
    * An assignment is a statement: a let statement can't be assigned to another variable 
    * A function definition is also a statement

* An **Expression** evaluates to a resultant value. Rust is an expression-based language.
    * A math operation is an expression. It evaluates to a value. --> the result of the operation
    * Calling functions also is an expression. 

* Functions with return values work like this: 
``` fn five() -> i32 { 5 }``` 
    * you could also write ```return 5```
    * do not put ;s in expressions like these

### Control flow

* if statements: 
``` if number < 5 { println!("Condition met"); } else { println!("Condition not met"); } ```
