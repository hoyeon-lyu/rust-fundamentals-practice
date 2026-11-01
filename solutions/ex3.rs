use std::io::{self, BufRead};

fn main(){
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let num1 = lines.next().unwrap().unwrap();
    let num2 = lines.next().unwrap().unwrap();
    let num3 = lines.next().unwrap().unwrap();

    let num1: i64 = num1.parse().unwrap();
    let num2: i64 = num2.parse().unwrap();
    let num3: i64 = num3.parse().unwrap();

    let mut total: i64 = 0;
    total += num1;
    println!("{}", total);
    total += num2;
    println!("{}", total);
    total += num3;
    println!("{}", total);

    println!("{} {}", total / 3, total % 3);
}