use std::collections::HashMap;
use std::io::{self, BufRead};

fn input() -> Vec<char> {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let return_vector = line.trim().split("")
                        .filter(|s| *s != "" && *s != " ")
                        .map(|s| s.to_lowercase().parse::<char>().unwrap())
                        .filter(|s| s.is_ascii_alphabetic())
                        .collect();
    return return_vector;
}

fn order_alphabetic(vector: &Vec<char>) -> Vec<char> {
    let alps: Vec<char> = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let mut return_vector: Vec<char> = Vec::new();
    for alp in alps {
        if vector.contains(&alp) {
            return_vector.push(alp);
        }
    }
    return return_vector;
}

fn main() {
    let mut count: HashMap<char, i64> = HashMap::new();
    let alphabets = input();
    for alphabet in alphabets {
        *count.entry(alphabet).or_insert(0) += 1;
    }
    
    let mut keys: Vec<char> = Vec::new();
    for key in count.keys() { keys.push(*key); }
    for key in order_alphabetic(&keys) {
        println!("{}: {}", key, count.get(&key).unwrap());
    }
}