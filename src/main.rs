use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let query = args
        .get(1)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing search query"))?;

    let filename = args
        .get(2)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing filepath"))?;

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let matching_lines = search(reader, query)?;

    for result in matching_lines {
        let line = result;
        println!("{line}")
    }

    Ok(())
}

fn search<R: BufRead>(reader: R, query: &String) -> io::Result<Vec<String>> {
    let mut matching_lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.contains(query) {
            matching_lines.push(line);
        }
    }

    Ok(matching_lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn finds_matching_lines() {
        let input = "hello rust\ngoodbye world\nhello again";
        let reader = Cursor::new(input);

        let results = search(reader, "hello").unwrap();

        assert_eq!(results.len(), 2);
    }
}
