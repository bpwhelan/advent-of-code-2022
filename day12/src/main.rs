use std::collections::{HashMap, VecDeque};

fn main() {
    let (cells, start, end) = parse_output();
    println!("Shortest Path Part 1: {}", part1(cells, start, end).unwrap());
    part2();
}

fn part1(cells: Vec<Cell>, start: usize, end: usize) -> Option<usize> {
    let mut step_queue: VecDeque<Cell> = VecDeque::new();
    let mut path_length: HashMap<usize, usize> = HashMap::new();
    let mut found = false;
    path_length.insert(start, 0);

    step_queue.push_front(cells[start].clone());

    while !step_queue.is_empty() {
        let current = step_queue.pop_front().unwrap();
        if current.id == end {
            found = true;
            break;
        }
        for sibling in current.siblings {
            if !path_length.contains_key(&sibling) || &(path_length.get(&current.id).unwrap() + 1) < path_length.get(&sibling).unwrap() {
                if cells[sibling].value - current.value <= 1 {
                    step_queue.push_back(cells[sibling].clone());
                    path_length.insert(sibling, path_length.get(&current.id).unwrap() + 1);
                }
            }
        }
    }
    if found {
        return Some(path_length.get(&end).unwrap().to_owned());
    }
    None
}

fn part2() {
    let (cells, _start, end) = parse_output();
    let mut a_paths = cells.iter().filter(|sq| sq.letter == 'a')
        .map(|f| part1(cells.clone(), f.id, end).unwrap_or(99999)).collect::<Vec<usize>>();

    a_paths.sort();

    println!("{:?}", a_paths);

}

// RECURSION FAIL
// fn step_cell(cell: Square, all_cells: Vec<Square>, mut visited: Vec<usize>, mut all_viable_paths: Vec<Vec<usize>>, end_cell: usize) -> Vec<Vec<usize>> {
//     visited.push(cell.id);
//     for sibling in cell.siblings {
//         if sibling == end_cell {
//             if all_cells[sibling].value <= cell.value + 1 {
//                 println!("{:?}", visited.len());
//                 all_viable_paths.push(visited.clone());
//             } else {
//                 continue;
//             }
//         }
//         if all_cells[sibling].value <= cell.value + 1 && !visited.contains(&(sibling)) {
//             all_viable_paths = step_cell(all_cells[sibling].clone(), all_cells.clone(), visited.clone(), all_viable_paths, end_cell);
//         }
//     }
//     return all_viable_paths;
// }

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn parse_output() -> (Vec<Cell>, usize, usize) {
    let mut ret: Vec<Cell> = Vec::new();
    let mut start_cell: usize = 0;
    let mut end_cell: usize = 0;
    let mut max_y = 0;
    for _ in util::file_utils::read_file_by_line() {
        max_y += 1;
    }
    let mut cell_iterator = 0;
    for (y,line) in util::file_utils::read_file_by_line().enumerate() {
        let line_str: String = line.unwrap();
        for (x, char) in line_str.chars().enumerate() {
            let mut siblings = Vec::new();
            if y != 0 { siblings.push(cell_iterator - line_str.len())}
            if y != max_y - 1 { siblings.push(cell_iterator + line_str.len())}
            if x != 0 { siblings.push(cell_iterator - 1)}
            if x != line_str.len() - 1 { siblings.push(cell_iterator + 1)}
            let mut cell = Cell { id: cell_iterator, letter: char, value: 0, siblings, x, y };
            match char {
                'S' => {
                    start_cell = cell_iterator;
                    cell.value = 0;
                }
                'E' => {
                    end_cell = cell_iterator;
                    cell.value = get_value_of_char('z') as isize;
                }
                _ => { cell.value = get_value_of_char(char) as isize }
            }
            ret.push(cell);
            cell_iterator += 1;
        }
    }
    (ret, start_cell, end_cell)
}

fn get_value_of_char(c: char) -> usize {
    return ALPHABET.chars().position(|r| r == c).unwrap() + 1;
}

#[derive(Clone, Debug, PartialEq)]
struct Cell {
    id: usize,
    letter: char,
    value: isize,
    siblings: Vec<usize>,
    x: usize,
    y: usize,
    // sibling_up: usize,
    // sibling_down: usize,
    // sibling_left: usize,
    // sibling_right: usize,
}
