use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::result;

const FILE_PATH: &str = "./rust_search_practice.txt";

fn main() -> io::Result<()> {
    let f = File::open(FILE_PATH)?;
    let mut pattern = String::new();
    let reader = BufReader::new(f);

    println!("Enter search term:");
    io::stdin().read_line(&mut pattern)?;

    let clean_pattern = pattern.trim();

    let matching_lines = search(reader, clean_pattern)?;

    for result in matching_lines {
        let line = result;
        println!("{line}")
    }

    Ok(())
}

fn search<R: BufRead>(reader: R, pattern: &str) -> io::Result<Vec<String>> {
    let mut matching_lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.contains(pattern) {
            matching_lines.push(line);
        }
    }

    Ok((matching_lines))
}
