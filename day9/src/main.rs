use std::str::FromStr;


fn main() {
    let mut visited = move_rope_through_input(2);
    println!("Part 1: {}", visited.len());

    visited = move_rope_through_input(10);
    println!("Part 2: {}", visited.len());
}

fn move_rope_through_input(knots: usize) -> Vec<Position> {
    let steps: Vec<Motion> = util::file_utils::read_file_by_line().map(|f| parse_output(f.unwrap())).collect();
    // let mut head = Position { x: 0, y: 0 };

    let mut knots: Vec<Position> = vec![Position { x: 0, y: 0}; knots];
    let mut visited: Vec<Position> = Vec::new();

    visited.push(knots[0].clone());


    for step in &steps {
        for _i in 0..step.steps {
            match step.direction {
                Direction::L => { knots[0].x -= 1 }
                Direction::R => { knots[0].x += 1 }
                Direction::U => { knots[0].y += 1 }
                Direction::D => { knots[0].y -= 1 }
            }
            for i in 1..knots.len() {
                let directions = get_knot_movement(&knots[i - 1], &knots[i]);
                for direction in directions {
                    match direction {
                        Direction::L => { knots[i].x -= 1 }
                        Direction::R => { knots[i].x += 1 }
                        Direction::U => { knots[i].y += 1 }
                        Direction::D => { knots[i].y -= 1 }
                    }
                }
            }
            let tail: &Position = knots.last().unwrap();
            if !visited.contains(tail) {
                visited.push(tail.clone());
            }
        }
    }
    visited
}

fn get_knot_movement(head: &Position, tail: &Position) -> Vec<Direction> {
    let mut directions = Vec::new();
    if head.x - 2 == tail.x {
        directions.push(Direction::R)
    }
    if head.y - 2 == tail.y {
        directions.push(Direction::U)
    }
    if head.x + 2 == tail.x {
        directions.push(Direction::L)
    }
    if head.y + 2 == tail.y {
        directions.push(Direction::D)
    }

    if directions.is_empty() {
        return directions;
    } else {
        if head.y > tail.y && !directions.contains(&Direction::U) {
            directions.push(Direction::U);
        }
        if head.y < tail.y && !directions.contains(&Direction::D) {
            directions.push(Direction::D);
        }
        if head.x > tail.x && !directions.contains(&Direction::R) {
            directions.push(Direction::R);
        }
        if head.x < tail.x && !directions.contains(&Direction::L) {
            directions.push(Direction::L);
        }
    }
    directions
}

fn parse_output(f: String) -> Motion {
    let split: Vec<&str> = f.split_ascii_whitespace().collect();
    Motion {
        direction: Direction::from_str(split[0]).unwrap(),
        steps: split[1].parse().unwrap(),
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    L,
    R,
    U,
    D,
}


impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "L" => Ok(Direction::L),
            "R" => Ok(Direction::R),
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Motion {
    direction: Direction,
    steps: usize,
}
