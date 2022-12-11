use std::fs::File;
use std::io::{BufRead, BufReader};

static FILE_PATH: &str = "input.txt";

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("In file {}", FILE_PATH);

    let reader = BufReader::new(File::open(FILE_PATH).expect("Cannot open file.txt"));

    let mut current_calorie_count = 0;

    let mut highest_calorie_count = 0;
    
    for line in reader.lines() {
        let calories: i32 = match line.unwrap().parse() {
            Ok(num) => num,
            Err(_) => {
                if current_calorie_count > highest_calorie_count {
                    highest_calorie_count = current_calorie_count;
                }
                current_calorie_count = 0;
                continue;
            }};
        current_calorie_count += calories;
    }

    println!("{}", highest_calorie_count)
}

fn part2() {
    println!("In file {}", FILE_PATH);

    let reader = BufReader::new(File::open(FILE_PATH).expect("Cannot open file.txt"));

    let mut elf_calories: Vec<i32> = Vec::new();
    let mut current_calorie_count = 0;
    
    for line in reader.lines() {
        let calories: i32 = match line.unwrap().parse() {
            Ok(num) => num,
            Err(_) => {
                elf_calories.push(current_calorie_count);
                current_calorie_count = 0;
                continue;
            }};
        current_calorie_count += calories;
    }

    elf_calories.sort();
    elf_calories.reverse();

    println!("{}", elf_calories[0] + elf_calories[1] + elf_calories[2]);
}
