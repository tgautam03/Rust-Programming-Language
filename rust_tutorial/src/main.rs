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

    println!("Hello {}, {}. What's your age?", name.trim_end(), greeting);

    let mut age:String = String::new();
    
    io::stdin()
        .read_line(&mut age)
        .expect("Age not entered");

    // Shadowing variable age
    let age:u32 = age.trim().parse()
                            .expect("Entered age is not a number!!");
    
    println!("{} is {} years old.", name.trim_end(), age);
}
