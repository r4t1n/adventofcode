use std::env::args;
use std::fs;
use std::process;

fn puzzle(input: &str) -> (u16, u16) {
    const NORTH: u8 = 0;
    const EAST: u8 = 1;
    const SOUTH: u8 = 2;
    const WEST: u8 = 3;
    let mut direction: u8 = NORTH;
    let mut x: i16 = 0;
    let mut y: i16 = 0;

    for instruction in input.split(", ") {
        let (turn, blocks) = instruction.split_at(1);
        let blocks: u8 = match blocks.parse() {
            Ok(b) => b,
            Err(_) => {
                println!("[!] Invalid block: {}", instruction);
                continue;
            }
        };

        match turn {
            "L" => direction = (direction + 3) % 4,
            "R" => direction = (direction + 1) % 4,
            _ => {
                println!("[!] Invalid turn: {}", instruction);
                continue;
            }
        }

        match direction {
            NORTH => y += blocks as i16,
            EAST => x += blocks as i16,
            SOUTH => y -= blocks as i16,
            WEST => x -= blocks as i16,
            _ => {
                println!("[!] Invalid instruction: {}", instruction);
                continue;
            }
        }
    }

    let blocks_away_part_1: u16 = (x.unsigned_abs()) + (y.unsigned_abs());
    let blocks_away_part_2: u16 = (x.unsigned_abs()) + (y.unsigned_abs());

    (blocks_away_part_1, blocks_away_part_2)
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
