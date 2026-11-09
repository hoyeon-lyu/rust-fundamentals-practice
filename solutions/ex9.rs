use std::io::{self, BufRead};

fn input() -> Vec<i64> {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let return_vector = line.trim().split_whitespace()
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect();
    return return_vector;
}

fn min(vector: &Vec<i64>) -> i64 {
    if vector.len() == 0 {
        return 0_i64;
    }
    let mut min_value = i64::MAX;
    for value in vector {
        if *value < min_value { min_value = *value; }
    }
    return min_value;
}

fn max(vector: &Vec<i64>) -> i64 {
    if vector.len() == 0 {
        return 0_i64;
    }
    let mut max_value = i64::MIN;
    for value in vector {
        if *value > max_value { max_value = *value; }
    }
    return max_value;
}

fn main() {
    let mut scores: Vec<i64> = input();
    println!("{}", scores.len());
    println!("{}", min(&scores));
    println!("{}", max(&scores));
    println!("{:.2}", scores.iter().sum::<i64>() as f64/ scores.len() as f64);
    scores.sort();
    let scores_str: Vec<String> = scores.iter().map(|s| s.to_string()).collect();
    println!("{}", scores_str.join(" "));
}