use std::env::args;
use std::fs;
use std::process;

fn puzzle(input: &str) -> (u32, u32) {
    let mut answer_part_1: u32 = 0;
    let mut answer_part_2: u32 = 0;

    for line in input.lines() {
        println!("{}", line);
        answer_part_1 += 1;
        answer_part_2 += 1;
    }

    (answer_part_1, answer_part_2)
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
