use std::ops::Range;

fn main() {
    let mut number_of_contained = 0;
    let mut number_of_overlapped = 0;
    for line in util::file_utils::read_file() {
        let mut range1: Vec<i32> = Vec::new();
        for elf in line.unwrap().as_str().split(",") {
            if !range1.is_empty() {
                let range2: Vec<i32> = elf.split("-").map(|s| s.parse::<i32>().unwrap()).collect();
                if (range1[0] >= range2[0] && range1[1] <= range2[1]) || (range2[0] >= range1[0] && range2[1] <= range1[1]) {
                    number_of_contained += 1;
                }
                if (range1[0] >= range2[0] && range1[0] <= range2[1]) || (range2[0] >= range1[0] && range2[0] <= range1[1]) {
                    number_of_overlapped += 1;
                }
            }
            range1 = elf.split("-").map(|s| s.parse::<i32>().unwrap()).collect();
        }
    }

    println!("{}", number_of_contained);

    println!("{}", number_of_overlapped);
}
