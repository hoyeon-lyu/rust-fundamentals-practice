use std::io::{self, BufRead};

fn points_input() -> i32 {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let points: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    points
}

fn main() {
    let points = points_input();
    let grade = match points {
        0..=59 => "F",
        60..=69 => "D",
        70..=79 => "C",
        80..=89 => "B",
        90..=100 => "A",
        _ => "Invalid"
    };
    println!("{}", grade);
}