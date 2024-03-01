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

### 2 - guessing game

* the standard library has a set of useful items described in the [standard library documentation](https://doc.rust-lang.org/stable/std/prelude/index.html) 
* create a variable with let
* VARIABLES are IMMUTABLE by default: to make a variable mutable you must add ```mut``` before the variable name 
    es: ``` let mut bananas = 5;```

