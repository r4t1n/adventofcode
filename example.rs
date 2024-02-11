use std::env::args;
use std::fs;
use std::process;

fn puzzle(input: &str) {
    println!("{}", input);
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
