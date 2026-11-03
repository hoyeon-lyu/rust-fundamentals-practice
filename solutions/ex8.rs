use std::io::{self, BufRead};

fn input() -> String {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let str_: String = lines.next().unwrap().unwrap();
    let return_value: String = String::from(str_.trim());
    return return_value;
}

fn count_digits(s: &str) -> usize {
    const DIGITS: &str = "0123456789";
    let mut counts = 0;
    for t in s.chars() {
        if DIGITS.contains(t) {
            counts += 1
        }
    }
    counts
}

fn remove_spaces(s: &mut String) {
    *s = s.replace(" ", "");
}

fn main() {
    let mut sentence: String = input();
    let before_sentence = sentence.clone();
    let count_digits = count_digits(&sentence);
    remove_spaces(&mut sentence);
    let after_sentence = sentence.clone();
    println!("{}", count_digits);
    println!("{}", after_sentence);
    println!("{} -> {}", before_sentence.len(), after_sentence.len());
}