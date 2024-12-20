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

    // Open the file and handle errors gracefully
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file '{}': {}", file_path, err);
            process::exit(1);
        }
    };

    // Use a BufReader to read the file line by line
    let reader = io::BufReader::new(file);
    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;

    // Iterate through the file line by line
    for line in reader.lines() {
        match line {
            Ok(line) => {
                line_count += 1;
                char_count += line.chars().count();
                word_count += line.split_whitespace().count();
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
                continue; // skip the problematic line
            }
        }
    }

    // Print the results
    println!("File Analysis for: {}", file_path);
    println!("-------------------------------");
    println!("Lines:     {}", line_count);
    println!("Words:     {}", word_count);
    println!("Characters: {}", char_count);
}
