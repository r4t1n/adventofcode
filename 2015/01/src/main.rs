use std::env::args;
use std::fs;
use std::process;

fn puzzle(input: &str) {
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
        } else if char == '\n' {
            continue;
        } else {
            println!("Character: '{}' is not valid (not a bracket)", char)
        }

        if floor == -1 && basement_position == 0 {
            basement_position = position;
        }
    }

    println!("Part 1: {}", floor);
    println!("Part 2: {}", basement_position)
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
