use std::env::args;
use std::fs::read_to_string;
use std::process::exit;

fn puzzle(input: &str) -> (u32, u32) {
    let mut grid_part_1: [[bool; 1000]; 1000] = [[false; 1000]; 1000];
    let mut grid_part_2: [[u32; 1000]; 1000] = [[0; 1000]; 1000];
    let mut lit_lights: u32 = 0;
    let mut total_brightness: u32 = 0;

    for line in input.lines() {
        let instruction: u8 = if line.starts_with("turn on") {
            1
        } else if line.starts_with("turn off") {
            0
        } else if line.starts_with("toggle") {
            2
        } else {
            println!("[!] Invalid line: {}", line);
            continue;
        };

        let coords: Vec<u16> = line
            .split(|c: char| !c.is_numeric())
            .filter_map(|s: &str| s.parse().ok())
            .collect();

        if coords.len() != 4 {
            println!("[!] Invalid line: {}", line);
            continue;
        }

        modify_grid(&mut grid_part_1, &mut grid_part_2, instruction, coords);
    }

    for row in grid_part_1.iter() {
        for &state in row.iter() {
            if state {
                lit_lights += 1;
            }
        }
    }

    for row in grid_part_2.iter() {
        for &brightness in row.iter() {
            total_brightness += brightness;
        }
    }

    (lit_lights, total_brightness)
}

fn modify_grid(
    grid_part_1: &mut [[bool; 1000]; 1000],
    grid_part_2: &mut [[u32; 1000]; 1000],
    instruction: u8,
    coords: Vec<u16>,
) {
    let instruction_part_1: fn(bool) -> bool = match instruction {
        1 => |_| true,
        0 => |_| false,
        2 => |state: bool| !state,
        _ => return,
    };

    let instruction_part_2: fn(u32) -> u32 = match instruction {
        1 => |brightness: u32| brightness + 1,
        0 => |brightness: u32| brightness.saturating_sub(1),
        2 => |brightness: u32| brightness + 2,
        _ => return,
    };

    for x in coords[0]..=coords[2] {
        for y in coords[1]..=coords[3] {
            grid_part_1[x as usize][y as usize] =
                instruction_part_1(grid_part_1[x as usize][y as usize]);
            grid_part_2[x as usize][y as usize] =
                instruction_part_2(grid_part_2[x as usize][y as usize]);
        }
    }
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
