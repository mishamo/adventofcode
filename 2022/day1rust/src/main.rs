use std::{env, io};
use std::fs::File;
use std::io::BufRead;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let file = File::open(file_path).expect("Should have been able to open file");
    let lines = io::BufReader::new(file).lines();
    let mut count: u32 = 0;
    let mut sums: Vec<u32> = Vec::new();
    for line in lines {
        match line {
            Ok(ln) => {
                if ln.is_empty() {
                    sums.push(count);
                    count = 0;
                } else {
                    let current_number: u32 = ln.parse().expect("Expected number");
                    count += current_number;
                }
            }
            Err(_) => exit(1)
        }
    }
    sums.sort();

    let total = match (sums.pop(), sums.pop(), sums.pop()) {
        (Some(a), Some(b), Some(c)) => a + b + c,
        _ => 0,
    };
    println!("{total}");
}
