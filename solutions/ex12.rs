use std::io::{self, BufRead};

enum Op {
    plus(i64, i64),
    minus(i64, i64),
    product(i64, i64),
    divide(i64, i64)
}

fn integer_check(operand: &str) -> Result<i64, String> {
    if operand.parse::<i64>().is_ok() { return Ok(operand.parse::<i64>().unwrap()) }
    else { return Err(String::from("not a number")) }
}

fn to_op(line: &str) -> Result<Op, String> {
    let line_vec: Vec<&str> = line.split_whitespace().collect();
    let operator = line_vec[1];

    let left_operand = match integer_check(line_vec[0]) {
        Ok(n) => n,
        Err(p) => return Err(p)
    };
    let right_operand = match integer_check(line_vec[2]) {
        Ok(n) => n,
        Err(p) => return Err(p)
    };

    let operation = match operator {
        "+" => Op::plus(left_operand, right_operand),
        "-" => Op::minus(left_operand, right_operand),
        "*" => Op::product(left_operand, right_operand),
        "/" => Op::divide(left_operand, right_operand),
        _ => return Err(String::from("unknown operator"))
    };

    return Ok(operation);
}

fn calc(line: &str) -> Result<i64, String> {
    let operation = match to_op(line) {
        Ok(n) => n,
        Err(p) => return Err(p)
    };

    match operation {
        Op::plus(left_operand, right_operand) => return Ok(left_operand + right_operand),
        Op::minus(left_operand, right_operand) => return Ok(left_operand - right_operand),
        Op::product(left_operand, right_operand) => return Ok(left_operand * right_operand),
        Op::divide(left_operand, right_operand) => { if right_operand == 0_i64 { return Err(String::from("division by zero")) }
                                                        else { return Ok(left_operand / right_operand) } }
    };
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let operation_ = line.unwrap();
        let operation = operation_.trim();
        match calc(operation) {
            Ok(n) => println!("ok: {}", n),
            Err(p) => println!("error: {}", p)
        };
    }
}