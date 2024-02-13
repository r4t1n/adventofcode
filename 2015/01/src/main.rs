use std::env::args;
use std::fs;
use std::process;

fn puzzle(input: &str) -> (i16, u16) {
    let mut basement_position: u16 = 0;
    let mut floor: i16 = 0;
    let mut position: u16 = 0;

    for char in input.chars() {
        if char == '(' {
            floor += 1;
            position += 1;
        } else if char == ')' {
            floor -= 1;
            position += 1;
        } else {
            println!("Character {} is not valid", char)
        }

        if floor == -1 && basement_position == 0 {
            basement_position = position;
        }
    }

    (floor, basement_position)
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("Pass the input file as the second argument");
        process::exit(1);
    }

    let input_filename: &String = &args[1];
    let input: String = match fs::read_to_string(input_filename) {
        Ok(content) => content.trim().to_string(),
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            process::exit(1);
        }
    };

    let (answer_part_1, answer_part_2) = puzzle(&input);
    println!("[+] Part 1: {}", answer_part_1);
    println!("[+] Part 2: {}", answer_part_2);
}
