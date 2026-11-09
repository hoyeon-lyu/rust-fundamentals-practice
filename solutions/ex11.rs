use std::io::{self, BufRead};

struct Student {
    name: String,
    score_korean: i32,
    score_english: i32,
    score_math : i32
}

impl Student {
    fn total(&self) -> i32{
        return self.score_korean + self.score_english + self.score_math;
    }

    fn average(&self) -> f64{
        return self.total() as f64 / 3.0 as f64;
    }
}

fn main() {
    let mut students: Vec<Student> = Vec::new();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: i32 = lines.next().unwrap().unwrap().trim().to_string().parse().unwrap();
    for _i in 0..n {
        let info_: String = lines.next().unwrap().unwrap();
        let info: Vec<&str> = info_.trim().split_whitespace().collect();
        students.push(Student { name: String::from(info[0]), score_korean: info[1].parse::<i32>().unwrap(), score_english: info[2].parse::<i32>().unwrap(), score_math: info[3].parse::<i32>().unwrap() });
    }
    
    let mut gt_student = &Student { name:String::from(""), score_korean:0, score_english:0, score_math:0 };
    for student in &students {
        if student.total() > gt_student.total() { gt_student = student;}
    }

    for student in &students {
        println!("{} {} {:.1}", student.name, student.total(), student.average());
    }
    println!("top: {}", gt_student.name);
}