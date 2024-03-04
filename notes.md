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

* ifs can be used on the left of an assignment
```let number = if condition { 5 } else { 6 };```
* if the ```if``` and the ```else``` arms have value types that are incompatible then Rust will print an error. Rust must know at compile time what type the number variable is, definitively. It cannot be defined at run-time

### Loops

* Three types of loops: ```loop```, ```while```, ```for```.

#### Loop loop

* it executes a block of code over and over again forever or until we explicitly tell it to stop. 
```fn main(){ loop { println!("again!"); }  };``` 
    * this will keep printing forever.
    * you can break out of the loop with, well, ```break;```
    * ```continue``` lets you skip the next part of the code to go directly to the next loop iteration.

* can be used for: 
    * event listener, checking whether a thread has completed its job, like a +1 counter on a variable external to the loop itself

* LOOP LABELS! 
``` 'counting: loop { 
        // etc etc etc
        loop {
            break 'counting_up;
        }
     }
```

#### While Loops

#### Ifs 
* only bool types.  


## OWNERSHIP -- unique to Rust

* a set of rules that govern how a rust program manages memory. 
Some language have garbage collection that regularly looks for no-longer-used memory as the program runs, in others the programmer must explicitly allocate and free the memory.
In Rust, memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won't compile, and none of ownership's feature will slow down the program while it's running. 

In Rust, whether a value is on the stack or on the heap affects how the language behaves and why you need to make certain decisions. 

Stack -> Last in, first out LIFO

### Ownership rules

1. Each value in Rust has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

#### String type

* String literals are immutable and they are fast and efficient when we need use fixed texts
* String type elements are mutable. Their memory gets allocated differently and we cant put a blob of memory into the binary for each piece of text whose size is unknown at compiletime and whose size might change while running the program. 
* Thus in order to support a mutable and growable piece of text, we allocate an amount of memory on the heap, unknown at compile time, to hold the contents. 
* THUS: 
    * The memory must be requested from the memory allocator at runtime
    * We need a way of returning this memory to the allocator once we're done with the string 

The first part is handled by us by calling ```String::from```. --> Requesting the memory it needs
The second part is different. Doesnt use garbage collector but doesnt use a manual way to free memory: 

**The memory is automatically returned once the variable that owns it goes out of scope.**
    HOLY FUCKING BINGLE THIS IS GENIUS?!

This means that when we copy a string to another string like ```let s1 = String::from("hello"); let s2 = s1;``` the string doesnt get copied but instead s2 points to the same index as s1 does, the index where the string hello is stored. 
Rust does not copy the heap data as well. If it did, the operation ```s2 = s1``` would be very expensive in terms of runtime performance with bigger data
But! When s2 and s1 go out of scope they would both free the same amount of memory leading to the double free error; freeing memory twice can lead to memory corruption --> security vulnerability 
Thus, Rust considers **s1 as no longer valid**. 
Thus instead of a copy, it's a *move*. s1 is *moved* s2. 

If we actually want a copy we can use a method called ```clone```. 
```
let s1 = String::from("hello");
let s2 = s1.clone();
println!("{}! {}!",s1, s2);
```

* In this way the heap data DOES get copied. Code may be expensive.

So what about the integer copy? It works and is valid because types such as integers have a known size at compile time and are stored entirely on the stack, so copies of the actual values are quick to make. Theres no need to clone. 

There is a special annotation called ```Copy``` for 'fixed' types, (types whose size are defined at compile time and do not undergo mutation); if ```Copy``` is used, variables that use it do not move but are trivially copied

Copy is implemented for 
    * All integers. The boolean type. All floating points. Char. Tuples, if they only contain types that also implement Copy. 

### REFERENCES 

* What if we want to let a functon use a value but not take ownership? The answer is REFERENCES!

``` let len = calc_len(&s1); 
//[...]
fn calc_len(s : &String) -> usize {
    s.len() 
}```

the syntax of a reference lets us create a reference that refers to the value of a variable but does not own it. Because it does not won it, the value it points to will not be dropped when the reference stops being used. 
* We call the action of creating a reference **borrowing**. 
* Just as variables are immutable by default, so are references! We can't modify something we have a reference to. 

* We CAN though use a mutable reference instead. 
* The limitation of using a mutable reference is that you can only have ONE reference if you use a mutable one! This is made to prevent data races. 
* It is possible to make a mutable reference variable to go out of scope: after a function scope is over, for example. We can use more mutable references, just not simultaneous ones.

#### Dangling references

* These are pointers that reference a location in memory that has been given to something else. This doesnt happen in Rust. If there is a reference to some data, the compiler ensures that the data will not go out of scope before the reference to the data does. 

```
let reference_to_nothing = dangle();
//[...]
fn dangle() -> &String {
let s = String::from("hello");
&s  }
```

* ^-- This isn't allowed. The code doesnt compile. 

* At any given time, you can have either one mutable reference OR any number of immutable references. 
* References must always be valid. 

### SLICES

&str = string literal, which is immutable 
``` let s = "Hello world";```  is a string literal 
```&s``` would be an immutable reference to a string literal 

* Summary: **Ownership**, **Borrowing*, **slices** --> ensure memory safety in Rust programs at **Compile Time**. The 'Owner' of the data automatically cleans up data when the owner goes out of scope --> dont need to write more code to free up memory because the compliler does this work for me. 

## STRUCTS AND STRUCTURES

* a structs lets you package multiple related values that make up a meaningful group. it's like an object's data attributes. 
* Different from tuples --> Here, you name each piece of data so it's clear what each thing means. You dont rely on the order of the data to access it. 
* functions can be associated to structs (and are thus called methods) 
* Structs and enums are the key building blocks for creating new types --> to use Rust's compile-time type checking.

* we access the value of the struct by using the dot notation. ```user1.email``` to access the email of user1. If the instance is mutable --> we can change a value by using the dot notation. 
* The entire instance must be mutable : Rust doesnt allow to mark only certain fields as mutable. 
