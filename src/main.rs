#![allow(unused)]

use std::io;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("What is your name?");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("didn't receive input"); // &mut -> directly saves value entered to name; read_line() returns enum
    // enum has fixed number of possible values, ok / error; error handling is built into rust using expect()
    // handling this error can be done too...

    let greeting: &str = "Nice to meet you!!";

    println!("Hello! {}!, {}", name.trim_end(), greeting); // {} is used to pass values directly; trim_end() removes newline char from end of name string


}
