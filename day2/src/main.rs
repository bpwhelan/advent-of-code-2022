use std::fs::File;
use std::io::{BufRead, BufReader};

static FILE_PATH: &str = "input.txt";
const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

fn main() {
    part1();
    part2();
}

fn part1() {

    println!("In file {}", FILE_PATH);

    let reader = BufReader::new(File::open(FILE_PATH).expect("Cannot open file.txt"));

    let mut current_score = 0;

    for line in reader.lines() {
        let line_str = line.unwrap();
        let vec: Vec<&str> = line_str.split_whitespace().collect();

        // println!("{:?}", vec);

        let them = match vec[0] {
            "A" => ROCK,
            "B" => PAPER,
            "C" => SCISSORS,
            _ => continue,
        };

        let you = match vec[1] {
            "X" => ROCK,
            "Y" => PAPER,
            "Z" => SCISSORS,
            _ => continue,
        };

        // println!("{}", you);

        let mut win = true;

        if you == them {
            current_score += 3;
        } else {
            if you == ROCK && them == PAPER {
                win = false;
            }

            if you == SCISSORS && them == ROCK {
                win = false;
            }

            if you == PAPER && them == SCISSORS {
                win = false;
            }
            if win {
                current_score += 6;
            }
        }

        current_score += you;
    }

    println!("{}", current_score);
}

fn part2() {
    println!("In file {}", FILE_PATH);

    let reader = BufReader::new(File::open(FILE_PATH).expect("Cannot open file.txt"));

    let mut current_score = 0;

    for line in reader.lines() {
        let line_str = line.unwrap();
        let vec: Vec<&str> = line_str.split_whitespace().collect();

        // println!("{:?}", vec);

        let them = match vec[0] {
            "A" => ROCK,
            "B" => PAPER,
            "C" => SCISSORS,
            _ => break,
        };

        let mut you= 0;

        let mut win = false;
        let mut tie = false;
        let mut lose = false;

        match vec[1] {
            "X" => lose = true,
            "Y" => tie = true,
            "Z" => win = true,
            _ => break,
        };

        if tie {
            you = them;
            current_score += 3;
        } else if win {
            you = match them {
                ROCK => PAPER,
                PAPER => SCISSORS,
                SCISSORS => ROCK,
                _=> break,
            };
            current_score += 6;
        } else if lose {
            you = match them {
                ROCK => SCISSORS,
                PAPER => ROCK,
                SCISSORS => PAPER,
                _=> break,
            };
        }
        current_score += you;
    }

    println!("{}", current_score);
}
