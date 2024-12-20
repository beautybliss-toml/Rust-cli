use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a file path is provided
    if args.len() != 2 {
        eprintln!("Usage: file_analyzer <file_path>");
        process::exit(1);
    }

    let file_path = &args[1];

    // Open the file
    let file = File::open(file_path).unwrap_or_else(|err| {
        eprintln!("Error opening file {}: {}", file_path, err);
        process::exit(1);
    });

    let reader = io::BufReader::new(file);
    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;

    // Iterate through the file line by line
    for line in reader.lines() {
        let line = line.unwrap();
        line_count += 1;
        char_count += line.chars().count();
        word_count += line.split_whitespace().count();
    }

    // Print the results
    println!("File: {}", file_path);
    println!("Lines: {}", line_count);
    println!("Words: {}", word_count);
    println!("Characters: {}", char_count);
}
