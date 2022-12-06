mod rucksack;
extern crate util;

use std::collections::{HashMap};
use util::file_utils;
use crate::rucksack::{build_sack, RuckSack};

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";


fn main() {
    let mut sacks: Vec<RuckSack> = Vec::new();

    for line in file_utils::read_file() {
        sacks.push(build_sack(line.unwrap()));
    }

    println!("Part 1: ");
    part1(&sacks);
    println!("Part 2: ");
    part2(&sacks);
}

fn part1(sacks: &Vec<RuckSack>) {
    let mut total_prio = 0;
    for sack in sacks {
        for c in sack.compartment1.chars() {
            if sack.compartment2.contains(c) {
                total_prio += get_prio(c);
                break;
            }
        }
    }
    println!("{}", total_prio);
}

fn part2(sacks: &Vec<RuckSack>) {
    let mut total_prio = 0;
    let sack_groups: Vec<&[RuckSack]> = sacks.chunks(3).collect();

    for sack_group in sack_groups {
        total_prio += get_prio(get_common_char(sack_group));
    }
    println!("{}", total_prio);
}

fn get_common_char(sack_group: &[RuckSack]) -> char {
    let mut map: HashMap<char, i32> = HashMap::new();

    for sack in sack_group {
        let mut visited_char: Vec<char> = Vec::new();
        for c in sack.full_sack.chars() {
            if visited_char.contains(&c) { continue; }
            match map.get(&c) {
                Some(count) => { map.insert(c, count + 1); }
                None => { map.insert(c, 1); }
            }
            visited_char.push(c);
        }
    }

    for entry in map {
        if entry.1 == sack_group.len() as i32 {
            return entry.0;
        }
    }
    panic!("no common char found!")
}

fn get_prio(c: char) -> usize {
    return ALPHABET.chars().position(|r| r == c).unwrap() + 1;
}
