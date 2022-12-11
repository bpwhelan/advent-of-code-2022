extern crate core;
use std::collections::HashMap;

fn part1(file_tree: &FileTree) -> usize {
    let mut total = 0;
    for (i, node) in file_tree.nodes.iter().enumerate() {
        if node.file_type.is_dir() {
            let size = file_tree.size(i);
            if size <= 100000 {
                total += size;
            }
        }
    }
    return total;
}

fn part2(file_tree: &FileTree) -> usize {
    let cur_used = file_tree.size(0);
    let cur_avail = 70000000 - cur_used;
    let want = 30000000 - cur_avail;
    let mut dirs: Vec<usize> = Vec::new();
    for (i, node) in file_tree.nodes.iter().enumerate() {
        if node.file_type.is_dir() {
            let size = file_tree.size(i);
            if size >= want {
                dirs.push(size)
            }
        }
    }
    dirs.sort();
    return dirs[0]
}


fn main() {
    let mut file_tree = FileTree::new();
    for line in util::file_utils::read_file_by_line() {
        let line_str = line.unwrap();
        let split: Vec<&str> = line_str.split_ascii_whitespace().collect();
        if line_str.starts_with("$ cd") {
            if line_str.contains(" ..") {
                file_tree.move_to_parent();
            } else {
                let dir_name = line_str.split("$ cd ").nth(1).unwrap();
                file_tree.find_and_move_to_child(dir_name);
            }
        } else if line_str.starts_with("dir") {
            file_tree.append_file(File {
                parent: Some(file_tree.cur),
                file_type: FileType::Dir { folder_name: split[1].to_owned(), children: vec![] },
            });
        } else if line_str.as_bytes()[0].is_ascii_digit() {
            file_tree.append_file(File {
                parent: Some(file_tree.cur),
                file_type: FileType::File { file_name: split[1].to_owned(), size: split[0].parse().unwrap() },
            })
        }
    }

    println!("{}", part1(&file_tree));
    println!("{}", part2(&file_tree));


    // let mut size_under_amount = 0;
    // for (dir, files) in &dirs {
    //     let size= get_dir_size(dir, files, &dirs);
    //     if size <= 100000 {
    //         size_under_amount += size;
    //     }
    // }

    // println!("{}", size_under_amount);
}

#[derive(Debug)]
enum FileType {
    Dir { folder_name: String, children: Vec<usize> },
    File { file_name: String, size: usize },
}

impl FileType {
    fn name(&self) -> &String {
        match self {
            FileType::Dir { folder_name, ..} => folder_name,
            FileType::File { file_name, .. } => file_name,
        }
    }

    #[must_use]
    fn is_dir(&self) -> bool {
        matches!(self, Self::Dir { .. })
    }
}

#[derive(Debug)]
struct File {
    parent: Option<usize>,
    file_type: FileType,
}

#[derive(Debug)]
struct FileTree {
    nodes: Vec<File>,
    cur: usize,
}

impl FileTree {
    fn size(&self, index: usize) -> usize {
        match &self.nodes[index].file_type {
            FileType::Dir { children, .. } => { children.iter().map(|&c| self.size(c)).sum() }
            FileType::File { size, .. } => { *size }
        }
    }

    fn new() -> Self {
        Self {
            nodes: vec![File { parent: None, file_type: FileType::Dir { folder_name: "/".to_string(), children: vec![] } }],
            cur: 0,
        }
    }

    fn find_and_move_to_child(&mut self, folder_name: &str) {
        match &self.nodes[self.cur].file_type {
            FileType::Dir { children, .. } => {
                for child in children {
                    if self.nodes[*child].file_type.name() == folder_name {
                        // cd to that child
                        self.cur = *child;
                    }
                }
            }
            FileType::File { .. } => {}
        }
    }

    fn move_to_parent(&mut self) {
        // println!("{:?}", self);
        self.cur = self.nodes[self.cur].parent.unwrap();
    }

    fn append_file(&mut self, file: File) {
        let node_index = self.nodes.len();
        self.nodes.push(file);
        let n = self.nodes.len();
        if n == 1 {
            return;
        }
        match &mut self.nodes[self.cur].file_type {
            FileType::Dir { ref mut children, .. } => { children.push(n - 1); }
            _ => {},
        }
    }
}


// unsafe fn get_dir_sizes(dir: &Box<Dir>) -> Vec<i32> {
//     let mut all_dir_sizes = Vec::new();
//     dir.sub_dir.iter().for_each(|sub_dir| { all_dir_sizes.append(&mut get_dir_sizes(sub_dir)); });
//     all_dir_sizes.push(get_dir_size(dir));
//     return all_dir_sizes;
// }
//
// unsafe fn get_dir_size(dir: &Box<Dir>) -> i32 {
//     let mut size = 0;
//     println!("GETTING DIR SIZE");
//     if dir.sub_dir.is_empty() {
//         dir.files.iter().for_each(|f| size += f.size);
//     } else {
//         dir.sub_dir.iter().for_each(|sub_dir| size += get_dir_size(sub_dir));
//     }
//     return size;
// }

// fn get_dir_size(dir: &String, files: &Vec<File>, dirs: &HashMap<String, Vec<File>>) -> i32 {
//     let mut size = 0;
//     // if "ffhwwg,hlqf,dsqwrdnq".contains(dir) {
//     //     println!("Folder : {}, Files {:?}", dir, files);
//     // }
//
//         for file in files {
//             if file.filename == "ffhwwg" {
//             if file.is_dir {
//                 if file.filename == "ffhwwg" {
//                     // println!("{}", file.filename);
//                 }
//                 match dirs.get(&file.filename) {
//                     None => {}
//                     Some(_) => {
//                         let dir_files = dirs.get(&file.filename).unwrap();
//                         size += get_dir_size(&file.filename, dir_files, dirs);
//                     }
//                 }
//             }
//             size += file.size;
//         }
//     }
//     return size;
// }

// #[derive(Debug)]
// struct File {
//     folder: String,
//     filename: String,
//     size: i32,
// }

// #[derive(Debug)]
// struct Dir {
//     parent: *mut Dir,
//     sub_dir: Vec<Box<Dir>>,
//     files: Vec<File>,
//     folder_name: String,
// }
//
// impl Dir {
//
//     fn new(folder_name: &str) -> Box<Dir> {
//         Box::new(Dir {
//             parent: std::ptr::null_mut(),
//             sub_dir: vec![],
//             files: vec![],
//             folder_name: folder_name.to_string(),
//         })
//     }
//
//     fn append_child(&mut self, mut child: Box<Dir>) -> usize {
//         child.parent = self;
//         self.sub_dir.push(child);
//         self.sub_dir.len()-1
//     }
//
//     fn append_file(&mut self, mut file: File) -> usize {
//         self.files.push(file);
//         self.files.len()-1
//     }
//
//     fn get_child_by_folder(&self, folder_name: &str) -> Option<&Box<Dir>> {
//         for child in &self.sub_dir {
//             if child.folder_name == folder_name {
//                 return Some(child);
//             }
//         }
//         None
//     }
//
//     // fn get_mut_child_by_folder(&self, folder_name: &str) -> Option<&mut Dir> {
//     //     for &mut child in &self.sub_dir {
//     //         if child.folder_name == folder_name {
//     //             Some(&mut child);
//     //         }
//     //     }
//     //     None
//     // }
//
//     // fn get_mut_child(&mut self, i:usize) -> Option<&mut Dir> {
//     //     if i < self.sub_dir.len() {
//     //         Some(&mut self.sub_dir[i])
//     //     } else {
//     //         None
//     //     }
//     // }
//
//     fn get_parent(&self) -> *mut Dir {
//         unsafe{ self.parent }
//     }
//
//     fn get_mut_parent(&mut self) -> Option<&mut Dir> {
//         unsafe{ self.parent.as_mut() }
//     }
// }
