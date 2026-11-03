use std::io::{self, BufRead};

fn time_input() -> i64 {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let time: i64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    time
}

fn main(){
    let time = time_input();
    let hours = time / 60_i64 / 60_i64;
    let minutes = time / 60_i64 % 60_i64;
    let seconds = time % 60_i64;
    println!("{:>02}:{:>02}:{:>02}", hours, minutes, seconds);
}