use std::{env, fs::File, io::{self, BufRead}};
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

    let blues = Regex::new(r"(\d+) blue").unwrap();
    let reds = Regex::new(r"(\d+) red").unwrap();
    let greens = Regex::new(r"(\d+) green").unwrap();

    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        if let Some(number) = process_cube_power_lines(&line, &blues, &reds, &greens) {
            sum += number;
            println!("{} -> {}", number, sum)
        }
    }

    println!("Sum of possible ids: {}", sum);
    Ok(())
}

fn process_cube_power_lines(line: &str, blues: &Regex, reds: &Regex, greens: &Regex) -> Option<i32> {
    let max_blues = find_max_cubes(line, blues);
    let max_reds = find_max_cubes(line, reds);
    let max_greens = find_max_cubes(line, greens);

    Some(max_blues * max_reds * max_greens)
}

fn find_max_cubes(line: &str, color_regex: &Regex) -> i32 {
    color_regex.captures_iter(line)
        .filter_map(|caps| caps.get(1))
        .filter_map(|match_| match_.as_str().parse::<i32>().ok())
        .max()
        .unwrap_or(0)
}

fn process_possible_game_line(line: &str) -> Option<i32> {
    let re = Regex::new(r"Game (\d+):").unwrap();
    return re.captures(line) // Apply the regex to the text
        .and_then(|caps| caps.get(1)) // Get the first capture group
        .map(|match_| match_.as_str().to_string())
        .map(|id|id.parse().unwrap())
        .filter(|id|is_game_possible(line));
}

fn is_game_possible(line: &str) -> bool {
    let blues = Regex::new(r"(\d+) blue").unwrap();
    let reds = Regex::new(r"(\d+) red").unwrap();
    let greens = Regex::new(r"(\d+) green").unwrap();

    let impossible_blues = is_possible(line, blues, 14);
    let impossible_reds = is_possible(line, reds, 12);
    let impossible_greens = is_possible(line, greens, 13);

    return !impossible_reds && !impossible_blues && ! impossible_greens
}

fn is_possible(line: &str, regex: Regex, threshold: i32) -> bool {
    regex.captures_iter(line)
        .filter_map(|caps| caps.get(1))
        .filter_map(|match_| match_.as_str().parse::<i32>().ok())
        .any(|num| num > threshold)
}