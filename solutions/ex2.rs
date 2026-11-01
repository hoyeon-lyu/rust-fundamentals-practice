use std::io::{self, BufRead};

fn main(){
    let stdin = io::stdin();
    let secs = stdin.lock().lines().next().unwrap().unwrap();

    let secs: i64 = secs.parse().unwrap();
    let hours: i64 = secs / 60 / 60;
    let mins: i64 = secs / 60 % 60;
    let secs: i64 = secs % 60;

    print!("{} hours {} minutes {} seconds", hours, mins, secs);
}