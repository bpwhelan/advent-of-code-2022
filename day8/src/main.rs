use crate::Direction::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

static DIRECTIONS: [Direction; 4] = [North, South, East, West];

#[derive(Debug, Clone)]
struct Tree {
    x: usize,
    y: usize,
    height: usize,
    visibility: Vec<Direction>,
}

impl Tree {
    fn new(x: usize, y: usize, height: char) -> Self {
        Tree { x, y, height: height as usize - '0' as usize, visibility: Vec::new() }
    }
}

#[derive(Debug, Clone)]
struct Forest {
    rows: Vec<Vec<Tree>>
}

impl Forest {
    fn get_height_of(&self, x: usize, y: usize) -> Option<usize> {
        Some(self.rows[y][x].height)
    }

    fn is_tree_visible_from(&self, tree: &Tree, start: usize, end: usize, constant: usize, dir: Direction) -> bool {
        if dir == North || dir == South {
            if dir == North {
                println!("start : {} : Tree : {:?}", start, tree);
            }
            for y in start..end {
                if self.get_height_of(constant, y).unwrap() >= tree.height {
                    return false;
                }
            }
        }

        if dir == East || dir == West {
            for x in start..end {
                if self.get_height_of(x, constant).unwrap() >= tree.height {
                    return false;
                }
            }
        }
        return true;
    }

    fn is_tree_visible(&self, tree: Tree, direction: Direction) -> bool {
        if match direction {
            North => { tree.y == 0 }
            South => { tree.y == self.rows.len() - 1 }
            East => { tree.x == self.rows[0].len() - 1 }
            West => { tree.x == 0 }
        } { return true; }
        match direction {
            North => { self.is_tree_visible_from(&tree, 0, tree.y, tree.x, direction) }
            South => { self.is_tree_visible_from(&tree, tree.y + 1, self.rows.len() - 1, tree.x, direction) }
            East => { self.is_tree_visible_from(&tree, tree.x + 1, self.rows[0].len(), tree.y, direction) }
            West => { self.is_tree_visible_from(&tree, 0, tree.x, tree.y, direction) }
        }
    }

    fn get_scenic_score(&self, tree: &Tree, start: usize, end: usize, constant: usize, dir: Direction) -> i32 {
        let mut trees_in_view = 0;

        match dir {
            North => {
                for y in (start..end).rev() {
                    trees_in_view += 1;
                    if self.get_height_of(constant, y).unwrap() >= tree.height {
                        break;
                    }
                }
            }
            South => {
                for y in start..end {
                    trees_in_view += 1;
                    if self.get_height_of(constant, y).unwrap() >= tree.height {
                        break;
                    }
                }
            }
            East => {
                for x in start..end  {
                    trees_in_view += 1;
                    if self.get_height_of(x, constant).unwrap() >= tree.height {
                        break;
                    }
                }
            }
            West => {
                for x in (start..end).rev()  {
                    trees_in_view += 1;
                    if self.get_height_of(x, constant).unwrap() >= tree.height {
                        break;
                    }
                }
            }
        }

        return trees_in_view;
    }

    fn get_scenic_score_of_tree(&self, tree: Tree, direction: Direction) -> i32 {
        let mut direction_score = 1;
        if match direction {
            North => { tree.y == 0 }
            South => { tree.y == self.rows.len() - 1 }
            East => { tree.x == self.rows[0].len() - 1 }
            West => { tree.x == 0 }
        } { return 1; }
        direction_score = match direction {
            North => { self.get_scenic_score(&tree, 0, tree.y, tree.x, direction) }
            South => { self.get_scenic_score(&tree, tree.y + 1, self.rows.len() - 1, tree.x, direction) }
            East => { self.get_scenic_score(&tree, tree.x + 1, self.rows[0].len(), tree.y, direction) }
            West => { self.get_scenic_score(&tree, 0, tree.x, tree.y, direction) }
        };
        direction_score
    }
}

fn main() {
    let mut forest: Forest =  Forest { rows: vec![vec![]] };

    for (y, line) in util::file_utils::read_file_by_line().enumerate() {
        for (x, height) in line.unwrap().chars().enumerate() {
            forest.rows[y].push(Tree::new(x, y, height));
        }
        forest.rows.push(vec![]);
    }

    let mut tree_counter = 0;
    for trees in forest.clone().rows {
        for mut tree in trees {
            for direction in DIRECTIONS {
                if forest.is_tree_visible(tree.clone(), direction) {
                    tree.visibility.push(direction);
                    println!("{:?}", tree);
                    tree_counter += 1;
                    break;
                }
            }
        }
    }

    let mut scores: Vec<i32> = Vec::new();
    for trees in forest.clone().rows {
        for mut tree in trees {
            let mut score = 1;
            for direction in DIRECTIONS {
                score *= forest.get_scenic_score_of_tree(tree.clone(), direction)
            }
            scores.push(score);
        }
    }

    scores.sort();
    scores.reverse();

    println!("{:?}", tree_counter);

    println!("{:?}", scores[0]);
}
