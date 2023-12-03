#![allow(unused)] // ignores unused vars at compile time

use std::{f32, io, usize};
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

    // vars
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = std::f32::consts::PI;
    let age: &str = "23";
    let mut age: u32 = age.trim().parse().expect("age wasn't assigned a number"); // building error handling directly as we code
    age = age + 1;
    println!("I'm {} age and I have money $ {}", age, ONE_MIL);

    // rust is statically typed; all types must be defined; may be auto-generated by compiler for you or explicit declaration if required
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);

    // if you have a variable that will be unused and you want rust compiler to ignore it, start with _
    let _is_true: bool = true;
    let my_cs_grade: char = 'A';
    let random_num = rand::thread_rng().gen_range(1..101); // [1, 100]
    println!("Random number: {}", random_num);

    // if-else
    if(age >= 18) && (age <= 60) {
        println!("not your retirement age");
    } else if (age >= 61) {
        println!("vacation");
    } else {
        println!("little you are!!");
    }

    // ternary
    let can_vote = if age >= 18 {true} else {false};
    println!("can vote?: {} ", can_vote);

    // match - runs different code depending on conditions
    let current_age: i32 = 23;
    match current_age {
        0..=18 => println!("you are young buddy !!"),
        19..=60 => println!("time to work !!"),
        61 | 22 => println!("party year !!"),
        62..=74 => println!("party year !!"),
        75..=i32::MAX => println!("bonus life !!"),
        _ => println!("not an important birthday !!")// match everthing else for current_age, helps you avoid errors
    }

    // match with cmp
    let voting_age: i32 = 18;
    match current_age.cmp(&voting_age) {
        Ordering::Less => println!("can't vote"),
        Ordering::Greater => println!("can vote"),
        Ordering::Equal => println!("you gained right to vote")
    }

    // elements in array must be of same type
    let arr_1: [i32; 4] = [1, 2, 3, 4];
    println!("first: {}, length: {}", arr_1[0], arr_1.len());
    let mut loop_idx = 0;
    loop { // while loop
        if loop_idx == 3 {
            break;
        }
        if arr_1[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_1[loop_idx] == 9 {
            break;
        }
        println!("val: {}", arr_1[loop_idx]);
        loop_idx += 1;
    }

    loop_idx = 0;
    while  loop_idx < arr_1.len() {
        println!("element: {}", arr_1[loop_idx]);
        loop_idx += 1;
    }

    for val in arr_1.iter() {
        println!("val: {}", val);
    }

    // tuples - can contain multiple data types of fixed size
    let my_tuple: (i32, f64, String) = (47, 50_068.090, "derek".to_string());
    println!("name: {}", my_tuple.2);
    let (v1, v2, v3) = my_tuple;
    println!("v1: {}", v1);

    // strings
    let mut str1 = String::new(); // []byte
    str1.push('A'); // at the end
    str1.push_str(" dam"); // at the end
    for word in str1.split_whitespace() {
        println!("word: {}", word);
    }

    let st2 = str1.replace("A", "Another");
    println!("updated string: {}", st2);

    let st3 = String::from("a d n c e");
    let mut vec1: Vec<char> = st3.chars().collect();
    vec1.sort();
    vec1.dedup();
    for char in vec1 {
        println!("{}", char);
    }


}
