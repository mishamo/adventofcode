mod main;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use std::cmp;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        return Ok(());
    }

    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;
    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        lines.push(line)
    }

    let digits = Regex::new(r"(\d+)").unwrap();
    for (line_number, line) in lines.iter().enumerate() {
        println!();
        println!("current line: {}", line);
        for caps in digits.captures_iter(line) {
            let match_ = caps.iter().skip(1) // Skip the entire match
                .filter_map(|m| m) // Filter out the None values
                .next() // Get the first capturing group that matched
                .unwrap(); // Unwrap the Option, safe because there's always at least one match

            let index = match_.start();
            let number = match_.as_str().parse::<i32>().ok();

            if let Some(num) = number {
                if include_number(num, index, line, &lines, line_number) {
                    sum += num;
                    println!("{} -> {}", num, sum)
                }
            }
        }
    }

    println!("{}", sum);

    return Ok(());
}

fn include_number(number: i32, index: usize, line: &String, lines: &Vec<String>, line_number: usize) -> bool {
    println!("current_number: {}", number);
    let num_digits = number.to_string().len();

    let mut start_index = index;
    if index == 0 {
        start_index = 0;
    } else {
        start_index = index.wrapping_sub(1);
    }

    let mut end_index = index + num_digits;
    if end_index != 140 {
        end_index += 1
    }

    let substring = &line[start_index..end_index];
    if start_index == 0 && !substring.ends_with(".") {
        // println!("1. start_index: {}, substring: {}", start_index, substring);
        return true;
    } else if end_index == 140 && !substring.starts_with(".") {
        // println!("2. end_index: {}, substring: {}", end_index, substring);
        return true;
    } else if start_index !=0 && end_index != 140 && !(substring.starts_with(".") && substring.ends_with(".")) {
        // println!("3. start_index: {}, end_index: {}, substring: {}", start_index, end_index, substring);
        return true;
    }

    let previous_line = lines.get(line_number.wrapping_sub(1));
    let next_line = lines.get(line_number+1);
    if let Some(previous) = previous_line {
        // println!("previous line: {}", previous);
        if contains_specific_chars(previous, index, index+num_digits) {
            return true;
        }
    }

    if let Some(next) = next_line {
        // println!("next line: {}", next);
        if contains_specific_chars(next, index, index+num_digits) {
            return true;
        }
    }

    return false;

}

fn contains_specific_chars(string: &str, start: usize, end: usize) -> bool {
    if start > end || end > string.len() {
        return false; // Return false if indices are out of bounds
    }

    let mut start_index = start;
    if start == 0 {
        start_index = 0;
    } else {
        start_index = start.wrapping_sub(1);
    }

    // println!("start: {}, end: {}", start_index, end);
    let substring = &string[start_index..cmp::min(end+1, 140)];
    // println!("searching {} in substring {} for characters", string, substring);
    substring.chars().any(|c| c != '.')
}