use std::env::args;
use std::fs;
use std::process;

fn puzzle(input: &str) {
    for char in input.chars() {
        println!("{}", char);
    }

    for line in input.lines() {
        println!("{}", line);
    }

    let x = 0;
    let y = 0;
    println!("[+] Part 1: {}", x);
    println!("[+] Part 2: {}", y);
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("Pass the input file as the second argument");
        process::exit(1);
    }

    let input_filename: &String = &args[1];
    let input: String = match fs::read_to_string(input_filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            process::exit(1);
        }
    };

    puzzle(&input);
}
