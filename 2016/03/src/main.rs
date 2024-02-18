use std::env::args;
use std::fs::read_to_string;
use std::process::exit;

// iterate over character and split for non numeric

fn puzzle(input: &str) -> (u16, i32) {
    let mut valid_triangles: u16 = 0;
    let answer_part_2 = 0;

    for line in input.lines() {
        let mut dimensions: Vec<u16> = line
            .split(|c: char| !c.is_numeric())
            .filter_map(|s: &str| s.parse().ok())
            .collect();

        if dimensions.len() != 3 {
            println!("[!] Invalid line: {}", line);
            continue;
        }

        if calculate_dimensions(&mut dimensions) {
            valid_triangles += 1;
        }
    }

    (valid_triangles, answer_part_2)
}

fn calculate_dimensions(dimensions: &mut [u16]) -> bool {
    dimensions.sort();

    (dimensions[0] + dimensions[1]) > dimensions[2]
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("[!] Usage: {} <input file>", args[0]);
        exit(1);
    }

    let input_filepath: &String = &args[1];
    let input: String = match read_to_string(input_filepath) {
        Ok(input) => input.trim().to_string(),
        Err(err) => {
            eprintln!("[!] Error reading input: {}", err);
            exit(1);
        }
    };

    let (answer_part_1, answer_part_2) = puzzle(&input);

    println!("[+] Part 1: {}", answer_part_1);
    println!("[+] Part 2: {}", answer_part_2);
}
