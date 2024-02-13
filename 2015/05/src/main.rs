use std::env::args;
use std::fs;
use std::process;

fn puzzle(input: &str) -> (u16, u16) {
    let mut nice_strings_part_1: u16 = 0;
    let mut nice_strings_part_2: u16 = 0;

    for line in input.lines() {
        if is_nice_part_1(line) {
            nice_strings_part_1 += 1;
        }

        if is_nice_part_2(line) {
            nice_strings_part_2 += 1;
        }
    }

    (nice_strings_part_1, nice_strings_part_2)
}

fn is_nice_part_1(line: &str) -> bool {
    let mut previous_char: Option<char> = None;
    let mut property_1: bool = false;
    let mut property_2: bool = false;
    let mut property_3: bool = false;
    let mut vowels: u8 = 0;

    for char in line.chars() {
        match char {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowels += 1;
                if vowels >= 3 {
                    property_1 = true;
                }
            }
            _ => {}
        }

        if let Some(previous) = previous_char {
            if previous == char {
                property_3 = true;
            }
        }

        previous_char = Some(char);
    }

    if !(line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy")) {
        property_2 = true;
    }

    property_1 && property_2 && property_3
}

fn is_nice_part_2(line: &str) -> bool {
    let bytes: &[u8] = line.as_bytes();
    let mut property_1: bool = false;
    let mut property_2: bool = false;

    for i in 0..bytes.len() - 1 {
        let pair: &str = &line[i..i + 2];
        let rest_of_string: &str = &line[i + 2..];

        if rest_of_string.contains(pair) {
            property_1 = true;
            break;
        }
    }

    for i in 0..bytes.len() - 2 {
        if bytes[i] == bytes[i + 2] {
            property_2 = true;
            break;
        }
    }

    property_1 && property_2
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
