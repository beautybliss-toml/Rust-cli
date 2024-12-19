use std::env;
use std::process;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if we have enough arguments
    if args.len() != 3 {
        eprintln!("Usage: add <num1> <num2>");
        process::exit(1);
    }

    // Parse the arguments into integers
    let num1: i32 = args[1].parse().unwrap_or_else(|err| {
        eprintln!("Error parsing number {}: {}", args[1], err);
        process::exit(1);
    });
    let num2: i32 = args[2].parse().unwrap_or_else(|err| {
        eprintln!("Error parsing number {}: {}", args[2], err);
        process::exit(1);
    });

    // Perform the addition
    let sum = num1 + num2;

    // Print the result
    println!("The sum of {} and {} is: {}", num1, num2, sum);
}
