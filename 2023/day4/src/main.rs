use std::collections::{HashMap, HashSet};
use std::io;
use std::fs;
use itertools::Itertools;

fn main() -> io::Result<()> {
    let contents: &str = &fs::read_to_string("input.txt").expect("Failure");
    let input = parse(contents);
    let part1 = part1(&input);
    println!("part1: {}", part1);
    let part2 = part2(&input);
    println!("part2: {}", part2);

    return Ok(())
}

fn part2(lines: &Vec<&str>) -> usize {
    let mut map = HashMap::new();
    for (line_number, line) in lines.iter().enumerate() {
        let winning_numbers = get_winning_numbers(line);
        // println!("{:?}", winning_numbers);
        let my_numbers = get_my_numbers(line);
        // println!("{:?}", my_numbers);

        let intersection = find_intersection(winning_numbers, my_numbers);
        // println!("{:?}", intersection);

        map.insert(line_number, intersection.len());
    }

    let mut new_map = HashMap::new();
    let keys: Vec<_> = map.keys().cloned().collect();
    for key in keys.iter().sorted() {
        let value = map.get(&key).unwrap();
        let start = key + 1;
        let end = key + 1 + value;
        for i in start..end {
            if i >= 203 {
                continue
            }
            let current_number_of_cards = new_map.get(key).unwrap_or(&1);
            for j in 0..*current_number_of_cards {
                let new_value = new_map.get(&i).unwrap_or(&1) + 1;
                new_map.insert(i, new_value);
            }
        }
    }

    for key in map.keys().cloned() {
        if !new_map.contains_key(&key) {
            new_map.insert(key, 1);
        }
    }

    println!("{:?}", map.iter().sorted());
    println!("{:?}", new_map.iter().sorted());
    let sum: usize = new_map.values().cloned().sum();

    return sum;
}

fn part1(lines: &Vec<&str>) -> i32 {

    let mut sum = 0;
    for line in lines {
        let winning_numbers = get_winning_numbers(line);
        println!("{:?}", winning_numbers);
        let my_numbers = get_my_numbers(line);
        println!("{:?}", my_numbers);

        let intersection = find_intersection(winning_numbers, my_numbers);
        println!("{:?}", intersection);

        let points = calculate_points(intersection);
        println!("{}", points);
        sum += points;
        println!("sum: {}", sum)
    }
    return sum
}

fn calculate_points(matches: Vec<i32>) -> i32 {
    if matches.is_empty() {
        return 0;
    }

    let base: i32 = 2;
    return base.pow((matches.len() - 1) as u32) as i32
}

fn find_intersection(first: Vec<i32>, second: Vec<i32>) -> Vec<i32> {
    let first_unique: HashSet<i32> = first.into_iter().collect();
    let second_unique: HashSet<i32> = second.into_iter().collect();

    first_unique.intersection(&second_unique).cloned().collect()
}

fn get_my_numbers(line: &str) -> Vec<i32> {
    let start = line.find("|").unwrap() + 1;
    let end = line.len();
    let numbers = &line[start..end];

    let split_numbers: Vec<&str> = numbers.split(" ").collect();

    split_numbers
        .iter()
        .map(|num|num.trim())
        .filter(|str|!str.is_empty())
        .map(|num|{
            num.parse::<i32>().unwrap()
        })
        .collect()
}

fn get_winning_numbers(line: &str) -> Vec<i32> {
    let start = line.find(":").unwrap() + 1;
    let end = line.find("|").unwrap();
    let numbers = &line[start..end];

    let split_numbers: Vec<&str> = numbers.split(" ").collect();

    split_numbers
        .iter()
        .map(|num|num.trim())
        .filter(|str|!str.is_empty())
        .map(|num|{
            num.parse::<i32>().unwrap()
        })
        .collect()
}

fn parse(filecontent: &str) -> Vec<&str> {
    filecontent.lines().collect()
}