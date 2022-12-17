use std::{i32, isize};
use std::ops::{Range, RangeInclusive};
use std::process::exit;
use std::thread::{current, Thread};

fn main() {
    // part1(parse_output());
    part2(parse_output())
}

fn part1(mut rocks_and_sand: Rocks) {
    println!("{:?}", rocks_and_sand);
    let mut max_y = rocks_and_sand.max_y;
    let mut sand_count = 0;
    let mut found = false;
    while !found {
        let mut x = 500;
        let mut y = 0;
        loop {
            let mut rest = false;
            match rocks_and_sand.get_sand_next_move(&Coord { x, y }) {
                Move::Rest => {
                    rocks_and_sand.add_sand(Coord { x, y });
                    rest = true;
                }
                Move::Down => {}
                Move::Left => { x -= 1 }
                Move::Right => { x += 1 }
            };
            if rest { break; }
            y += 1;
            if y >= max_y {
                found = true;
                break;
            }
        }
        if found {break;}
        sand_count += 1;
    }
    println!("{}", sand_count)
}

fn part2(mut rocks_and_sand: Rocks) {
    rocks_and_sand.add_floor(rocks_and_sand.max_y + 2);
    println!("{:?}", rocks_and_sand);
    let mut max_y = rocks_and_sand.max_y + 2;
    let mut sand_count = 0;
    let mut found = false;
    while !found {
        sand_count += 1;
        let mut x = 500;
        let mut y = 0;
        loop {
            let mut rest = false;
            match rocks_and_sand.get_sand_next_move(&Coord { x, y }) {
                Move::Rest => {
                    if x == 500 && y == 0 {
                        println!("REST AT {}", sand_count);
                        exit(0);
                    }
                    rocks_and_sand.add_sand(Coord { x, y });
                    rest = true;
                }
                Move::Down => {}
                Move::Left => { x -= 1 }
                Move::Right => { x += 1 }
            };
            if rest { break; }
            y += 1;
            if y >= max_y {
                found = true;
                break;
            }
        }
        if found {break;}
        if sand_count % 500 == 0 {
            println!("SAND POURED : {}", sand_count);
        }
    }
    println!("{}", sand_count)
}


fn parse_output() -> Rocks {

    let mut ret: Rocks = Rocks {
        rocks: vec![],
        max_y: 0,
    };
    for line in util::file_utils::read_file_by_line() {
        ret.append(Rocks::parse(line.unwrap().as_str()))
    }
    ret
}

#[derive(Debug, Clone, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Rocks {
    rocks: Vec<Coord>,
    max_y: usize,
}

impl Coord {
    fn parse(s: &str) -> Self {
        let mut tokens = s.split(",");
        let (x, y) = (tokens.next().unwrap(), tokens.next().unwrap());
        Self { x: x.parse().unwrap(), y: y.parse().unwrap() }
    }
}

enum Move {
    Rest,
    Down,
    Left,
    Right
}

impl Rocks {
    fn add_floor(&mut self, max_y: usize) {
        self.append(Rocks::parse(format!("300,{} -> 700,{}", max_y, max_y).as_str()))
    }

    fn append(&mut self, mut rocks: Rocks) {
        self.rocks.append(&mut rocks.rocks);
        if rocks.max_y > self.max_y { self.max_y = rocks.max_y }
    }
    fn add_sand(&mut self, sand: Coord) {
        self.rocks.push(sand);
    }
    fn check_for_rock(&self, x:usize, y:usize) -> bool {
        self.rocks.contains(&Coord {x, y})
    }

    fn get_sand_next_move(&self, sand: &Coord) -> Move {
        let candidates = vec![
            Coord {x: sand.x, y: sand.y + 1},
            Coord {x: sand.x - 1, y: sand.y + 1},
            Coord {x: sand.x +1, y: sand.y + 1}
        ];
        if !self.check_for_rock(sand.x, sand.y + 1) {
            Move::Down
        } else if !self.check_for_rock(sand.x - 1, sand.y + 1) {
            Move::Left
        } else if !self.check_for_rock(sand.x + 1, sand.y + 1) {
            Move::Right
        } else {
            Move::Rest
        }
    }

    fn parse(s: &str) -> Self {
        let coords = s.split(" -> ").map(Coord::parse).collect::<Vec<Coord>>();

        println!("{:?}", coords);
        let mut iter = coords.iter().peekable();

        let mut rocks: Vec<Coord> = Vec::new();
        let mut prev = None;
        let mut current = None;
        let mut max_y = 0;
        while iter.peek().is_some() {
            match prev {
                None => {
                    prev = iter.next();
                    current = iter.next();
                }
                Some(_) => {
                    prev = current.clone();
                    current = iter.next();
                }
            }
            if prev.unwrap().x != current.unwrap().x {
                for x in get_ordered_range(prev.unwrap().x, current.unwrap().x) {
                    let coord = Coord { x, y: prev.unwrap().y  };
                    if !rocks.contains(&coord) {
                        rocks.push(coord )
                    }
                }
            } else if prev.unwrap().y != current.unwrap().y {
                for y in get_ordered_range(prev.unwrap().y,current.unwrap().y) {
                    let coord = Coord { x: prev.unwrap().x, y };
                    if !rocks.contains(&coord) {
                        rocks.push(coord)
                    }
                }
            }
            if current.unwrap().y > max_y { max_y = current.unwrap().y }
        }
        Self { rocks, max_y }
    }
}

fn get_ordered_range(start: usize, end: usize) -> RangeInclusive<usize> {
    let mut v = [start, end];
    if start > end { v.reverse() }
    v[0]..=v[1]
}