use std::env::args;
use std::fs::read_to_string;
use std::process::exit;

fn puzzle(input: &str) -> (u16, u16) {
    let mut dimensions_sets: Vec<Vec<u16>> = vec![vec![0; 3], vec![0; 3], vec![0; 3]];
    let mut line_count: u8 = 0;
    let mut possible_triangles_part_1: u16 = 0;
    let mut possible_triangles_part_2: u16 = 0;

    for line in input.lines() {
        let mut dimensions_line: Vec<u16> = line
            .split(|c: char| !c.is_numeric())
            .filter_map(|s: &str| s.parse().ok())
            .collect();

        if dimensions_line.len() != 3 {
            println!("[!] Invalid line: {}", line);
            continue;
        }

        for (i, dim_set) in dimensions_sets.iter_mut().enumerate() {
            dim_set[line_count as usize] = dimensions_line[i];
        }

        line_count = (line_count + 1) % 3;

        if is_possible_triangle(&mut dimensions_line) {
            possible_triangles_part_1 += 1;
        }

        if line_count == 0 && dimensions_sets.iter().all(|set| set.len() == 3) {
            for dim_set in &mut dimensions_sets {
                if is_possible_triangle(dim_set) {
                    possible_triangles_part_2 += 1;
                }
            }
        }
    }

    (possible_triangles_part_1, possible_triangles_part_2)
}

fn is_possible_triangle(dimensions: &mut [u16]) -> bool {
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
    println!("[+] Reading input from: {}", input_filepath);

    let input: String = match read_to_string(input_filepath) {
        Ok(input) => input.trim().to_string(),
        Err(err) => {
            eprintln!("[!] Error reading input: {}", err);
            exit(1);
        }
    };

    println!("[+] Solving the puzzle...");

    let (answer_part_1, answer_part_2) = puzzle(&input);

    println!("[*] Part 1: {}", answer_part_1);
    println!("[*] Part 2: {}", answer_part_2);
}
