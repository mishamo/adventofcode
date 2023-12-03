use std::{env, io};
use std::fs::File;
use std::io::BufRead;
use regex::Regex;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        return Ok(());
    }

    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        lines.push(line)
    }

    let digits_regex = Regex::new(r"(\d+)").unwrap();

    let mut sum = 0;
    for (line_number, line) in lines.iter().enumerate() {
        println!();
        println!("Current line: {}", line);

        for (char_index, char) in line.chars().enumerate() {
            if char == '*' {
                if let (Some(num)) = search_for_numbers(&lines, line_number, char_index, &digits_regex) {
                    println!("Ratio: {}", num);
                    sum += num;
                }
            }
        }
    }

    println!("{}", sum);
    return Ok(());
}

fn search_for_numbers(lines: &Vec<String>, line_number: usize, index: usize, digits_regex: &Regex) -> Option<i32> {
    let previous_line = lines.get(line_number.wrapping_sub(1));
    let next_line = lines.get(line_number + 1);

    let mut numbers = Vec::new();

    if let Some(previous) = previous_line {
        let mut nums = find_numbers_around_index(previous, index, digits_regex);
        numbers.append(& mut nums);
    }

    if let Some(next) = next_line {
        let mut nums = find_numbers_around_index(next, index, digits_regex);
        numbers.append(& mut nums);
    }

    let mut nums = find_numbers_around_index(lines.get(line_number).unwrap(), index, digits_regex);
    numbers.append(& mut nums);

    if numbers.len() == 2 {
        println!("nums: {:?}", numbers);
        return Some(numbers.get(0).unwrap() * numbers.get(1).unwrap())
    }
    return None
}

fn find_numbers_around_index(line: &String, index: usize, digits_regex: &Regex) -> Vec<i32> {
    let mut numbers = Vec::new();
    for caps in digits_regex.captures_iter(line) {
        let match_ = caps.iter().skip(1)
            .filter_map(|m| m)
            .next().unwrap();

        let start_index = match_.start();
        let number_as_string = match_.as_str();
        let end_index = start_index + number_as_string.len();
        let number = number_as_string.parse::<i32>().ok().unwrap();

        if (index - 1 < end_index && index - 1 >= start_index) ||
            (index <= end_index && index >= start_index) ||
            (index + 1 <= end_index && index + 1 >= start_index) {
            numbers.push(number)
        }

    }
    return numbers;
}
