# The Rust Programming Language
[[2024-08-04]] @ 17:11 
This file serves as my log as I work through "The Rust Programming Language" book.

[The Book](https://rust-book.cs.brown.edu/) refers to a book called "The Rust Programming Language". There are several versions of the book;
https://doc.rust-lang.org/stable/book/
This is the the book in a fairly raw format, a clean series of HTML documents for each chapter.
https://rust-book.cs.brown.edu/
The brown university version has interactive quizzes for each chapter.

## Intro
[[2024-08-04]] @ 17:02 
### Tools
#### Cargo
#definition **Cargo:** Dependency manager and build tool
#### Rustfmt 
#definition **rustfmt:** formatting tool
#### rust-analyser
#definition **rust-analyser:** IDE integration for code completion and inline error messages

## getting started
[[2024-08-04]] @ 18:17 
### Installation & maintenance
```bash
# update rust
rustup update

# show rust version
rustc --version
# show cargo version
cargo --version

# rust docs
rustup doc

# uninstall rust
rustup self uninstall
```

### Hello, World!
[[2024-08-04]] @ 18:20 
wrote a print statement in `hello_world/`[[Projects/Active/Learning Rust/learningRustProjects/hello_world/main.rs|main]]:
```rust
fn main(){
    println!("Hello, world!");
}
```

using the terminal within the project folder `hello_world/` I compiled the script with:
```bash
rustc main.rs
```

This produces a binary executable of the same name, this can then be run with:
```bash
./main
```

### Hello, Cargo!
[[2024-08-04]] @ 18:28 
This time I will use `cargo` instead of `rustc`.

The project is created with `cargo new <project name>`, this creates a `Cargo.toml` file for package & dependency control as well as an `src/` folder containing a `main.rs` file for your code. 
	Supposedly it also creates a git repo along with a .gitignore however as my learning projects are withing my Obsidian vault which itself is a repo it didn't.

The project can then be built and run with `cargo run`, 
	or alternatively built but not run with `cargo build`, or checked with the compiler without building an executable with `cargo check`.

## Guessing Game
[[2024-08-04]] @ 19:03 
For the first project I make a guessing game.

### Taking in a Guess
as before creating a new project with `cargo new`.

Contents of [[Projects/Active/Learning Rust/learningRustProjects/guessing_game/src/main.rs|main]]

```rust title:"Number Guessing Game"
// This project requires a dependency from the standard library `std` called `io` for inputs and outputs, this is included by the following declaration.
use std::io;

// as before the main function is declared, which contains all the code we expect to run at run time
fn main() {

    // print function as used in [[#Hello, World!]]
    println!("guesss the number:"); 

    // we define the mut (mutable) variable "guess" as an empty string
    // the empty string is created with the "String" associated function "new()"
    let mut guess = String::new();

    // we ustilise the "stdin()" funciton from the "io" library 
    io::stdin()
    
        // we append the input to the "guess" variable
        .read_line(&mut guess)
        
        // ".expect()" is an error handling funciton it takes in the result enum and if it is of type "Err" (error) it prints the argument to console
        .expect("Failed to read line");

    // this print statmetn uses a placeholder "{}" this is replaced by a variable
    println!("You guessed {}", guess);
}
```

%%
> [!NOTE] stopping point [[2024-08-04]] @ 19:44 
> https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html
%%
### Checking the Guess
[[2024-08-07]] @ 10:40 
As I moved on I immediately started breaking things into functions to see how it worked.

```rust title:"Number Guessing Game"
use std::io;
use rand::Rng;

fn get_guess() -> i32 {
    loop {
        println!("Make a guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");
        match guess.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Well it's defenitly not that")
        }
    }
}
fn check_guess(target: i32, guess: i32) -> bool {
    if guess == target {
        println!("I don't want to play anymore");
        true
    }
    else{
        println!("Ha rubbish, your like {} off!", target-guess);
        false
    }
}

fn main() {
    println!("Welcome to The Guessing Game!\n\n   I (the game) will generate a random number between 1 and and a billion,\nthats a one with 9 zeros mind you not 12, to win the game you must guess this number.\n\n Ready\n\n Set!\n\n GO!\n");
    let target = rand::thread_rng().gen_range(1..=1000000000);
    loop{
        let guess = get_guess();
        if check_guess(target,guess){
            break;
        }
    }
}
```

%%
> [!NOTE] Stopping point [[2024-08-08]] @ 01:04 
> https://doc.rust-lang.org/stable/book/ch03-00-common-programming-concepts.html
%%
## Data Types
[[2024-08-08]] @ 17:25 
*In the book this section is called [Common Programming Concepts](https://doc.rust-lang.org/stable/book/ch03-00-common-programming-concepts.html).*

Data types can be broken down in several ways. Initially we will consider [[#Variables and Mutability]] then go through the [[#Primitive Data Types]].

### Variables and Mutability
[[2024-08-08]] @ 17:25 
[[#Variables]] and [[#Constants]] only exist within the scope they where defined in i.e. the bounds of their surrounding `{ }`.

#### Variable
#definition **Variables:** are stores of values that are **computed at run time**.

Variables are defined with the keyword `let`.

By default variables are immutable, the compiler will throw an error if you try to reassign them without fully redefining the variable [see Shadowing](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html#shadowing). 

Variables can be made mutable with the prefix `mut`.

#### Constant
#definition **Constants:** are stores of values that are **computed at compile time**.

Constants are defined with the keyword `const`.

### Primitive Data Types
[[2024-08-08]] @ 18:00 
Often [[#Data Types]] don't need to be declared explicitly, Rust can often figure it out through context, though sometimes it is necessary to set types explicitly.

Data types can be thought of in 2 categories:
1. [[#Scalar Types]] - [[#Variable]]s that store a single value
2. [[#Compound Types]] - [[#Variable]]s that store a list of values

#### Scalar Types
There are 4 primitive scalar types:
1. [[#Integer]]
2. [[#Float]]
3. [[#Bool]]
4. [[#Char]]

##### Integer
[Integer Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#integer-types) are defined by `i` or `u` for signed and unsigned respectively, followed by the number of bits.

| Length  | Signed  | Unsigned |
| ------- | ------- | -------- |
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

##### Float
#definition [Floating-Point Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#floating-point-types) cab be either `f32` or `f64` for the number of bits, however there's rarely any point in using `f32` Rust will default to `f64`.

Floats are always signed.

If a [[#Variable]] is defined with a numeric containing a decimal point (i.e. 
`{rust}let x = 2.0;`) it will default to a `f64` type without needing an explicit keyword.

##### Bool
#definition [The Boolean Type](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#the-boolean-type) is defined with the keyword `bool` and must only be `true` or `false` it cannot be assigned with `1`,`0`,`yes`,`no`,etc.

A [[#Variable]] defined as a `true` or `false` (i.e. `{rust}let t = true;`) will default to a `bool` type without needing an explicit keyword.

##### Char
#definition [The Character Type](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#the-character-type) is a primitive alphabetical type storing one character as a 4 byte Unicode Scalar Value. 

#### Compound Types
[[2024-08-08]] @ 18:28 
There are 2 primitive [[#Compound Types]]:
1. [[#Tuple]]
2. [[#Array]]

##### Tuple
#definition [The Tuple Type](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#the-tuple-type) stores lists of values with **different** [[#Data Types]].

Tuples have a **fixed length**, once defined they cannot grow or shrink.

Tuples are defined with comma delineated lists in parentheses "`()`" i.e.
`{rust}let tup = (500, 6.4, 1);`
To explicitly define the types of elements in a tuple use "`:`" followed by a comma delineated lists in parentheses "`()`" containing the [[#Data Types]] in the respective positions i.e.:
`{rust}let tup: (i32, f64, u8) = (500, 6.4, 1);`

Elements from a [[#Tuple]] can be referenced by a "." followed by their position in the list i.e.:
`{rust}<tup_name>.<0_indexed_position>`
`{rust}tup.0`
##### Array
[The Array Type](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#the-array-type) stores lists of values with the **same** [[#Data Types]].

Arrays have a **fixed length**, once defined they cannot grow or shrink.

Arrays are defined with comma delineated lists in square brackets "`[]`" i.e.
`{rust}let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];`
To explicitly define the [[#Data Types]] use "`:`" followed by a comma delineated lists in square brackets "`[]`" with the type followed by the length i.e.
`{rust}let a: [i32; 5] = [1, 2, 3, 4, 5];`

To reference elements in an array use the arrays name followed by the element position in square brackets i.e.:
`{rust}<array_name>[<0_indexed_position>]`
`{rust}a[0]` 

%%
> [!NOTE] Stopping point [[2024-08-08]] @ 19:04 
> https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html
%%
### Functions
[[2024-08-11]] @ 11:10 
Functions are declared with the keyword `fn` and convention is to use snake case, all lower case and separated by `_`.

The `main()` function is called at runtime.

#### Parameters
Function parameters are defined inside parentheses "`()`" with their name followed by "`:`" then their type. Multiple parameters are delineated by "`,`". See the example below:

```rust
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

#### Statements
#definition **Statements:** are instructions that perform some action and do not return a value.

#### Expressions
#definition **Expressions:** evaluate to a resultant value.

unlike most lines expressions do not end with a semicolon "`;`".

Within functions Expressions act like the `return` keyword and the `return` keyword can in fact be used in rust it's just not required.

### Comments
[[2024-08-11]] @ 11:30 
Simple comments are added with `//` from then on the text in that line is treated as a comment.

### Control Flow
[[2024-08-11]] @ 11:32 

#### `if` Expressions
Logical branches can be created with `if` and `else` statements. Code block contained by their respective "`{}`" are refereed to as "arms". 

`else if` statements tell rust to only run the first `true` condition of the set, whereas `if` will run all `true` conditions. 

##### Using `if` inside a `let` statement
if using a branch to set a value inside a `let` statement the types must match.
#### Loops

##### Loop Termination
The `continue` keyword tells Rust to skip the rest of the code in the current iteration and start again in the next. 

the `break` keyword tells Rust to break out of the loop at that point. 

These keywords act on the innermost active loop.

##### `loop`
Loops indefinitely
##### `while`
Loops conditionally 

%%
> [!NOTE] Stopping point [[2024-08-11]] @ 11:49 
> https://doc.rust-lang.org/stable/book/ch04-00-understanding-ownership.html
%%
## Ownership
[[2024-08-13]] @ 17:08 
### Stack vs Heap
[[2024-08-13]] @ 17:48 
The Stack and the Heap are different parts of memory.
#### Stack
#definition **The stack:** is is a portion of memory a program can utilise at runtime. It is **rigidly ordered** such that the **last in** is the **first out** so items must be pushed to the stack in the opposite order that they will be needed. It is also a **fixed size** all data sizes must be set at compile time.

##### Features 
- Fast
- Rigid

#### Heap
#definition **The heap:** is memory a program can utilise at runtime. It is allocated dynamically, when called for the memory allocator finds an empty spot large enough for the request and returns a location pointer. The location pointers can be stored on the [[#Stack]].
##### Features
- Dynamic
- Slower

### Ownership Rules
[[2024-08-14]] @ 09:40 
- Each value in Rust has an _owner_.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

%%
> [!NOTE] Stopping Point [[2024-08-14]] @ 09:53
> https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html
%%
