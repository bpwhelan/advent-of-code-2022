use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Add, Div, Mul};
use std::str::FromStr;

fn main() {
    print!("Part 1 : ");
    pass_between_monkeys(20, 3);
    print!("Part 2 : ");
    pass_between_monkeys(10000, 0);
}

fn pass_between_monkeys(rounds: usize, manual_relief: usize) {
    let mut monkeys = parse_output();

    let mut inspection_level: HashMap<usize, usize> = HashMap::new();

    let mut lcm= monkeys[0].test.divisible_by;
    for i in 1..monkeys.len() {
        lcm = find_lcm(lcm,monkeys[i].test.divisible_by)
    }

    for _round in 0..rounds {
        for monkey_index in 0..monkeys.len() {
            for mut _item_index in 0..monkeys[monkey_index].items.len() {
                match inspection_level.get(&monkey_index) {
                    Some(count) => { inspection_level.insert(monkey_index, count + 1); }
                    None => { inspection_level.insert(monkey_index, 1); }
                }
                let mut value = monkeys[monkey_index].operation.value;
                if monkeys[monkey_index].operation.use_old_as_value {
                    value = monkeys[monkey_index].items[0];
                }
                let divide_by = (&monkeys[monkey_index]).test.divisible_by;
                let mut worry_level = match monkeys[monkey_index].operation.operator {
                    Operator::Mult => { monkeys[monkey_index].items[0].mul(value)}
                    Operator::Add => { monkeys[monkey_index].items[0].add(value)}
                    };

                if manual_relief != 0 {
                    worry_level = worry_level.div(manual_relief);
                } else {
                    worry_level = worry_level % lcm;
                }

                if worry_level.divisible_by(monkeys[monkey_index].test.divisible_by) {
                    let move_to = (&monkeys[monkey_index]).test.if_true;
                    monkeys[move_to].items.push(worry_level);
                    monkeys[monkey_index].items.remove(0);
                } else {
                    let move_to = (&monkeys[monkey_index]).test.if_false;
                    monkeys[move_to].items.push(worry_level);
                    monkeys[monkey_index].items.remove(0);
                }
            }
        }
    }
    let mut inspection_counts: Vec<usize> = inspection_level.values().cloned().collect::<Vec<usize>>();
    inspection_counts.sort();
    inspection_counts.reverse();


    println!("{:?}", inspection_counts[0] * inspection_counts[1]);
}

fn find_lcm(a: usize, b: usize) -> usize {
    a * b / gcd_euclid(a, b)
}

fn gcd_euclid(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    return gcd_euclid(b, a % b)
}


fn parse_output() -> Vec<MonkeyHas> {
    let mut monkeys: Vec<MonkeyHas> = Vec::new();

    let mut items: Vec<usize> = Vec::new();
    let mut operation: Operation = Operation::new();
    let mut test: Test = Test::new();


    for line in util::file_utils::read_file_by_line() {
        let line_str = line.unwrap();
        // can just use vec index
        // if line_str.contains("Monkey ") {
        //     monkey = line_str.replace(":", "").split_ascii_whitespace().collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
        // } else
        if line_str.contains("Starting") {
            items.append(&mut line_str.split(":").last().unwrap().split_ascii_whitespace().map(|f| {
                let f2 = f.replace(",", "");
                f2.parse::<usize>().unwrap()
            }).collect::<Vec<usize>>());
        } else if line_str.contains("Operation"){
            let operation_split: Vec<&str> = line_str.split("new = old ").last().unwrap().split_ascii_whitespace().collect();
            let use_old = operation_split[1] == "old";
            operation = Operation {
                operator: Operator::from_str(operation_split[0]).unwrap(),
                value: if use_old {0} else { operation_split[1].parse().unwrap()},
                use_old_as_value: use_old,
            }
        } else if line_str.contains("Test") {
            test.divisible_by = line_str.split_ascii_whitespace().last().unwrap().parse().unwrap();
        } else if line_str.contains("If true:") {
            test.if_true = line_str.split_ascii_whitespace().last().unwrap().parse().unwrap();
        } else if line_str.contains("If false:") {
            test.if_false = line_str.split_ascii_whitespace().last().unwrap().parse().unwrap();
        }

        if line_str == "" {
            monkeys.push(
                MonkeyHas {
                    items,
                    operation,
                    test,
                }
            );
            items = Vec::new();
            operation = Operation::new();
            test = Test::new();
        }
    }
    monkeys.push(
        MonkeyHas {
            items,
            operation,
            test,
        }
    );
    monkeys
}


#[derive(Debug, PartialEq)]
struct MonkeyHas {
    items: Vec<usize>,
    operation: Operation,
    test: Test,
}

#[derive(Debug, PartialEq)]
struct Operation {
    operator: Operator,
    value: usize,
    use_old_as_value: bool,
}

#[derive(Debug, PartialEq)]
enum Operator {
    Mult,
    Add,
}

#[derive(Debug, PartialEq)]
struct Test {
    divisible_by: usize,
    if_true: usize,
    if_false: usize,
}

impl Operation {
    fn new() -> Self {
        Operation {
            operator: Operator::Add,
            value: 0,
            use_old_as_value: false,
        }
    }
}

impl Test {
    fn new() -> Self {
        Test {
            divisible_by: 0,
            if_true: 0,
            if_false: 0,
        }
    }
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(input: &str) -> Result<Operator, Self::Err> {
        match input {
            "*" => Ok(Operator::Mult),
            "+" => Ok(Operator::Add),
            _ => Err(()),
        }
    }
}

trait DivisibleBy {
    fn divisible_by(&self, divisor: usize) -> bool;
}

impl DivisibleBy for usize {
    fn divisible_by(&self, divisor: usize) -> bool {
        return self % divisor == 0;
    }
}