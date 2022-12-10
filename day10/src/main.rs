use std::str::FromStr;

fn main() {
    part1();

    part2();
}

fn part1() {
    let mut instructions = parse_input();
    let mut cycle = 0;
    let mut x = 1;
    let mut total = 0;
    for instruction in instructions {
        let mut cur_cycle = 0;
        match instruction.instruction {
            InstructionType::Noop => { cur_cycle = 1 }
            InstructionType::Addx => {
                cur_cycle = 2;
            }
        }
        for i in 0..cur_cycle {
            cycle += 1;
            if cycle == 20 || (cycle - 20) % 40 == 0 {
                // println!("{}", cycle);
                total += x * cycle;
            }
            if i == 1 {
                x += instruction.value;
            }
        }

    }
    println!("{}", total);
}

fn part2() {
    let mut instructions = parse_input();
    let mut cycle = 0;
    let mut x = 1;
    for instruction in instructions {
        let mut cur_cycle = 0;
        match instruction.instruction {
            InstructionType::Noop => { cur_cycle = 1 }
            InstructionType::Addx => {
                cur_cycle = 2;
            }
        }
        for i in 0..cur_cycle {
            if cycle % 40 == 0 {
                println!()
            }
            if x - 1 <= (cycle % 40) && x + 1 >= (cycle % 40) {
                print!("#")
            } else { print!(".") }
            if i == 1 {
                x += instruction.value;
            }
            cycle += 1;
        }

    }
}

fn parse_input () -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in util::file_utils::read_file_by_line() {
        let line_str = line.unwrap();
        let split: Vec<&str> = line_str.split_ascii_whitespace().collect();
        instructions.push(Instruction {
            instruction: InstructionType::from_str(split[0]).unwrap(),
            value: if split.len() > 1 { split[1].parse().unwrap() } else { 0 },
        })
    }
    return instructions;
}

#[derive(Debug)]
struct Instruction {
    instruction: InstructionType,
    value: isize,
}

#[derive(Debug, PartialEq)]
enum InstructionType {
    Noop,
    Addx,
}

impl FromStr for InstructionType {
    type Err = ();

    fn from_str(input: &str) -> Result<InstructionType, Self::Err> {
        match input {
            "noop" => Ok(InstructionType::Noop),
            "addx" => Ok(InstructionType::Addx),
            _ => Err(()),
        }
    }
}