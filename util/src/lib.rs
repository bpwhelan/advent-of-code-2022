pub mod file_utils {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Lines};

    static FILE_PATH: &str = "input";
    pub fn read_file() -> Lines<BufReader<File>> {
        println!("In file {}", FILE_PATH);

        let reader = BufReader::new(File::open(FILE_PATH).expect("Cannot open file.txt"));

        return reader.lines();
    }
}