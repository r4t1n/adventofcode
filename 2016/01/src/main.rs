use std::collections::HashSet;
use std::env::args;
use std::fs::read_to_string;
use std::process::exit;

fn puzzle(input: &str) -> (u16, u16) {
    const NORTH: u8 = 0;
    const EAST: u8 = 1;
    const SOUTH: u8 = 2;
    const WEST: u8 = 3;
    let mut blocks_away_part_2: u16 = 0;
    let mut direction: u8 = NORTH;
    let mut visited: HashSet<(i16, i16)> = HashSet::new();
    let mut x: i16 = 0;
    let mut y: i16 = 0;

    visited.insert((0, 0));

    for instruction in input.split(", ") {
        let (turn, blocks) = instruction.split_at(1);
        let blocks: u8 = match blocks.parse() {
            Ok(blocks) => blocks,
            Err(_) => {
                println!("[!] Invalid block: {}", instruction);
                continue;
            }
        };

        match turn {
            "L" => direction = (direction - 1) % 4,
            "R" => direction = (direction + 1) % 4,
            _ => {
                println!("[!] Invalid turn: {}", instruction);
                continue;
            }
        }

        for _ in 0..blocks {
            match direction {
                NORTH => y += 1,
                EAST => x += 1,
                SOUTH => y -= 1,
                WEST => x -= 1,
                _ => {
                    println!("[!] Invalid instruction: {}", instruction);
                    continue;
                }
            }

            if !visited.insert((x, y)) && blocks_away_part_2 == 0 {
                blocks_away_part_2 = x.unsigned_abs() + y.unsigned_abs();
            }
        }
    }

    let blocks_away_part_1: u16 = x.unsigned_abs() + y.unsigned_abs();

    (blocks_away_part_1, blocks_away_part_2)
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

    println!("[+] Running the puzzle...");

    let (answer_part_1, answer_part_2) = puzzle(&input);

    println!("[*] Part 1: {}", answer_part_1);
    println!("[*] Part 2: {}", answer_part_2);
}
