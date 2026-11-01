use std::io::{self, BufRead};

fn main(){
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let name = lines.next().unwrap().unwrap();
    let age = lines.next().unwrap().unwrap();

    let name = name.trim();
    let age: i64 = age.trim().parse().unwrap();
    println!("Hi, {}!", name);
    println!("Next year you will be {}.", age + 1_i64);
}