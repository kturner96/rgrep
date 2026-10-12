use std::fs::File;
use std::io::BufReader;
use std::io::{self, Read};

const FILE_PATH: &str = "./rust_search_practice.txt";

fn main() -> io::Result<()> {
    let f = File::open(FILE_PATH).expect("File not found!");
    let mut pattern = String::new();
    let mut contents = String::new();
    let mut reader = BufReader::new(f);

    reader
        .read_to_string(&mut contents)
        .expect("Unable to load contents.");

    println!("Enter search term:");
    io::stdin()
        .read_line(&mut pattern)
        .expect("Pattern not valid.");

    let clean_pattern = pattern.trim();

    search(&contents, clean_pattern);

    Ok(())
}

// fn read_file() -> String {
//     fs::read_to_string(FILE_PATH).expect("Unable to read file.")
// }

fn search(contents: &str, pattern: &str) {
    for line in contents.lines() {
        if line.contains(pattern) {
            println!("{line}")
        }
    }
}
