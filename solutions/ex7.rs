use std::io::{self, BufRead};

fn number_input() -> i32 {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let number: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    number
}

fn main() {
    let number = number_input();
    let mut number_now: i32 = number;
    let mut steps: i32 = 0;
    loop {
        print!("{}", number_now);
        if number_now == 1 { break; }
        if number_now % 2 == 0 { number_now /= 2; }
        else if number_now % 2 != 0 { number_now = 3 * number_now + 1; }
        print!(" ");
        steps += 1;
    }
    println!("\nsteps: {}", steps);
}