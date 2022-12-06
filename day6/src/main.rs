use std::time::Instant;

fn main() {
    let input = util::file_utils::read_file_to_string();

    let time = Instant::now();
    find_start(input.clone(), 4);
    println!("Elapsed : {:.2?}", time.elapsed());

    let time = Instant::now();
    find_start(input.clone(), 14);
    println!("Elapsed : {:.2?}", time.elapsed());
}

fn find_start(input: String, marker_length: i32) {
    let mut buffer: String = String::new();

    for (index, c) in input.chars().enumerate() {

        let mut found = false;
        buffer.push(c);

        for (i, char) in buffer.chars().enumerate() {
            for j in i + 1..buffer.len() {
                if char == buffer.as_bytes()[j] as char {
                    found = true;
                }
            }
        }

        if !found && buffer.len() == marker_length as usize {
            println!("beginning of packet is {}", index + 1);
            break;
        } else if buffer.len() == marker_length as usize {
            buffer.remove(0);
        }
    }
}
