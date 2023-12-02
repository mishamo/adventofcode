use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn find_digits_in_line(line: &str) -> Vec<(usize, char)> {
    let number_words = [
        ("zero", '0'), ("one", '1'), ("two", '2'), ("three", '3'), ("four", '4'),
        ("five", '5'), ("six", '6'), ("seven", '7'), ("eight", '8'),
        ("nine", '9'),
    ];

    let mut positions = Vec::new();

    for (word, digit) in number_words.iter() {
        let mut start = 0;
        while let Some(pos) = line[start..].find(word) {
            let actual_pos = start + pos;
            positions.push((actual_pos, *digit));
            start = actual_pos + 1; // Move past the current match
        }
    }

    positions.sort_by_key(|&(pos, _)| pos);
    positions
}
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
    for line in reader.lines() {
        let line = line?;

        if let Some(number) = process_line(&line) {
            sum += number;
            println!("{} -> {}: {}", &line, number, sum)
        }
    }

    println!("Sum of calibration values: {}", sum);
    Ok(())
}

fn process_line(line: &str) -> Option<i32> {
    let mut positions = find_digits_in_line(line);

    // Append positions of standalone digits
    for (i, ch) in line.chars().enumerate() {
        if ch.is_digit(10) {
            positions.push((i, ch));
        }
    }

    positions.sort_by_key(|&(pos, _)| pos);

    let first_digit = positions.first().map(|&(_, digit)| digit);
    let last_digit = positions.last().map(|&(_, digit)| digit);

    match (first_digit, last_digit) {
        (Some(first), Some(last)) => {
            let combined = format!("{}{}", first, last);
            combined.parse::<i32>().ok()
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_line() {
        assert_eq!(process_line("1abc2"), Some(12));
        assert_eq!(process_line("pqr3stu8vwx"), Some(38));
        assert_eq!(process_line("a1b2c3d4e5f"), Some(15));
        assert_eq!(process_line("treb7uchet"), Some(77));
        assert_eq!(process_line("12345"), Some(15));
        assert_eq!(process_line("no digits"), None);
        assert_eq!(process_line("two1nine"), Some(29));
        assert_eq!(process_line("eightwothree"), Some(83));
        assert_eq!(process_line("abcone2threexyz"), Some(13));
        assert_eq!(process_line("xtwone3four"), Some(24));
        assert_eq!(process_line("4nineeightseven2"), Some(42));
        assert_eq!(process_line("zoneight234"), Some(14));
        assert_eq!(process_line("7pqrstsixteen"), Some(76));
        assert_eq!(process_line("oneighthree"), Some(13));
        assert_eq!(process_line("49threenjdgrmgfnfhcgz"), Some(43));
        assert_eq!(process_line("fourmsmjqfmbjvtwosevendcljsdcstl3one"), Some(41));
        assert_eq!(process_line("onezvbhrblrkzcrsevensix96jnpxjone"), Some(11));
    }
}