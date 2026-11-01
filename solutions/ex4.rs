use std::io::{self, BufRead};

fn main(){
    let stdin = io::stdin();
    let sen = stdin.lock().lines().next().unwrap().unwrap();
    let sen: &str = sen.trim();

    let sen_lower: String = sen.to_lowercase();
    let sen_chars = sen.chars().count();
    let sen_bytes = sen.len();
    let is_eq: bool = sen_chars == sen_bytes;

    println!("{}", sen_lower);
    println!("{}", sen_chars);
    println!("{}", sen_bytes);
    println!("{}", is_eq);
}