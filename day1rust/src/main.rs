use std::{env, io};
use std::fs::File;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let file = File::open(file_path).expect("Should have been able to open file");
    let lines = io::BufReader::new(file).lines();
    let mut count: i32 = 0;
    let mut sums: Vec<i32> = Vec::new();
    for line in lines {
        if let Ok(ln) = line {
            if ln.is_empty() {
                sums.push(count);
                count = 0;
            } else {
                let current_number: i32 = ln.parse().unwrap();
                count += current_number;
            }
        }
    }
    sums.sort();

    let total = match (sums.pop(), sums.pop(), sums.pop()) {
        (Some(a), Some(b), Some(c)) => a + b + c,
        _ => 0,
    };
    println!("{}", total);
}
