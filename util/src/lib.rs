pub mod file_utils {
    use std::fs;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Lines};

    static FILE_PATH: &str = "input";
    pub fn read_file_by_line() -> Lines<BufReader<File>> {
        println!("In file {}", FILE_PATH);

        let reader = BufReader::new(File::open(FILE_PATH).expect("Cannot open file.txt"));

        return reader.lines();
    }

    pub fn read_file_to_string() -> String {
        println!("In file {}", FILE_PATH);

        return fs::read_to_string(FILE_PATH).unwrap()
    }
}