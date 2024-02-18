use std::env::args;
use std::fs::read_to_string;
use std::process::exit;

fn puzzle(input: &str) -> (i16, u16) {
    let mut basement_position: u16 = 0;
    let mut floor: i16 = 0;
    let mut position: u16 = 0;

    for char in input.chars() {
        match char {
            '(' => {
                floor += 1;
                position += 1;
            }
            ')' => {
                floor -= 1;
                position += 1;
            }
            _ => {
                println!("[!] Invalid character: {}", char);
                continue;
            }
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
        println!("[!] Usage: {} <input file>", args[0]);
        exit(1);
    }

    let input_filename: &String = &args[1];
    let input: String = match read_to_string(input_filename) {
        Ok(input) => input.trim().to_string(),
        Err(err) => {
            eprintln!("[!] Error reading file: {}", err);
            exit(1);
        }
    };

    let (answer_part_1, answer_part_2) = puzzle(&input);

    println!("[+] Part 1: {}", answer_part_1);
    println!("[+] Part 2: {}", answer_part_2);
}
