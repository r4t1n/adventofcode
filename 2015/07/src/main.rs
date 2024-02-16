use std::collections::HashMap;
use std::env::args;
use std::fs;
use std::process;

fn puzzle(input: &str) -> (i32, i32) {
    let mut gates: HashMap<String, u16> = HashMap::new();
    let answer_part_1: i32 = 0;
    let answer_part_2: i32 = 0;

    for line in input.lines() {
        let operation: &str = if line.contains("NOT") {
            "NOT"
        } else if line.contains("AND") {
            "AND"
        } else if line.contains("OR") {
            "OR"
        } else if line.contains("LSHIFT") {
            "LSHIFT"
        } else if line.contains("RSHIFT") {
            "RSHIFT"
        } else {
            ""
        };

        let wires: Vec<u16> = line
        .split(|s: &str| )
        .filter_map(|s: &str| s.parse().ok())
        .collect();

        if operation.is_empty() {
            println!("Empty");
        }
    }

    (answer_part_1, answer_part_2)
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("[!] Usage: {} <input file>", args[0]);
        process::exit(1);
    }

    let input_filename: &String = &args[1];
    let input: String = match fs::read_to_string(input_filename) {
        Ok(content) => content.trim().to_string(),
        Err(err) => {
            eprintln!("[!] Error reading file: {}", err);
            process::exit(1);
        }
    };

    let (answer_part_1, answer_part_2) = puzzle(&input);
    println!("[+] Part 1: {}", answer_part_1);
    println!("[+] Part 2: {}", answer_part_2);
}
