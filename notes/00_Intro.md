# Introduction
The very first to do with Rust is to create a new project using cargo:
```sh
$ cargo new rust_tutorial
     Created binary (application) `rust_tutorial` package
```
This command creates a directory named *rust_tutorial* with the following structure:
```
rust_tutorial
    |- Cargo.toml
    |- src
        |- main.rs
```

*Cargo.toml* is just a config file for configuring the package with dependencies.

## User I/O
```rust
#![allow(unused)] // Hide warning for unused variables

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("What is your name?"); // println is a macro as it ends with !
    let mut name:String = String::new(); // Mutable string variable
    let greeting:&str = "Nice to Meet you";

    // Reading input from cmd line
    io::stdin()
        .read_line(&mut name)
        .expect("No input received!!");

    println!("Hello {}, {}", name.trim_end(), greeting);
}
```

- `String` is a dynamic heap string type while `str` is allocated on stack (this is usually defined in the program before compiling). With `String` we use `" "` but with `char` we use `' '`. 
- `io::stdin()` returns `Stdin` which has a `read_line` function defined accepting mutable `String` type variable. This in turn returns a `Result<usize, Error>` value. Function `expect` can catch the error (if any) and display the desired message which is passed as an argument.

> If we start the variable name with underscore like `let _name = ...;`, compiler will not warm if this variable is unused.

## Variables
### Constants
Constants are defined in uppercase letters as follows:
```rust
const ONE_MILLION:u32 = 1_000_000;
const PI:f32 = 3.141592;
```

### Shadowing
It is legal to re-define a variable with different type:
```rust
fn main() {
    .
    .
    .

    let mut age:String = String::new();
    
    io::stdin()
        .read_line(&mut age)
        .expect("Age not entered");

    // Shadowing
    let age:u32 = age.trim().parse()
                            .expect("Entered age is not a number!!");
    
    println!("{} is {} years old.", name.trim_end(), age);
}
```

> `parse()` also returns a `Result`.

## Math