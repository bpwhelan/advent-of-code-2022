use std::borrow::Borrow;
use std::collections::{HashMap, LinkedList};
use std::num::ParseIntError;
use std::usize;

fn main() {
    let mut all_cargo: Vec<LinkedList<char>> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();

    for j in 0..9 {
        all_cargo.push(LinkedList::new());
    }
    for line in util::file_utils::read_file_by_line() {
        let line_string = line.unwrap();
        if line_string.starts_with("[") {
            for (i, c) in line_string.chars().enumerate() {
                if c.is_uppercase() {
                    println!("{} : {}", c, i/4);
                    all_cargo[i/4].push_back(c);
                }
            }
        } else if line_string.starts_with("move") {
            let mut parsed: Vec<usize> = Vec::new();
            for entry in line_string.split_ascii_whitespace() {
                match entry.parse::<usize>() {
                    Ok(_) => { parsed.push(entry.parse::<usize>().unwrap())}
                    Err(_) => {}
                }
            }
            instructions.push(Instruction { amount: parsed[0], from: parsed[1] - 1, to: parsed[2] - 1});
        }
    }

    // part1(&all_cargo, &instructions);

    part2(all_cargo, instructions);

}

// fn part1(all_cargo: Vec<LinkedList<char>>, instructions: Vec<Instruction>) {
//     println!("{:?}", all_cargo);
//
//     for instruction in instructions {
//         println!("move {} from {} to {}",instruction.amount, instruction.from + 1, instruction.to + 1);
//         for i in 0..instruction.amount {
//             let from_char = all_cargo[instruction.from].pop_front().unwrap();
//             all_cargo[instruction.to].push_front(from_char);
//
//             println!("{:?}", all_cargo);
//
//             // println!("{}", instruction.from);
//             // println!("{:?}", all_cargo);
//         }
//     }
//
//     for cargo in all_cargo {
//         print!("{}", cargo.front().unwrap());
//     }
//
//     // for cargo in all_cargo[0].iter() {
//     //     println!("{:?}", &cargo);
//     // }
//
//     // println!("{:?}", all_cargo);
// }

fn part2(mut all_cargo: Vec<LinkedList<char>>, instructions: Vec<Instruction>) {
    println!("{:?}", all_cargo);

    for instruction in instructions {
        println!("move {} from {} to {}",instruction.amount, instruction.from + 1, instruction.to + 1);
        let mut boxes: Vec<char> = Vec::new();
        for i in 0..instruction.amount {
            boxes.push(all_cargo[instruction.from].pop_front().unwrap());
            // println!("{}", instruction.from);
            // println!("{:?}", all_cargo);
        }
        boxes.reverse();
        for char in boxes {
            all_cargo[instruction.to].push_front(char);
        }
    }

    for cargo in all_cargo {
        print!("{}", cargo.front().unwrap());
    }

    // for cargo in all_cargo[0].iter() {
    //     println!("{:?}", &cargo);
    // }

    // println!("{:?}", all_cargo);
}

#[derive(Debug)]
struct Cargo {
    letter: char,
    position: usize,
}

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

