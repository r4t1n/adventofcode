use std::collections::HashMap;
use std::env::args;
use std::fs;
use std::process;

fn puzzle(input: &str) -> (u32, u32) {
    let mut grid_part_1: HashMap<(u16, u16), bool> = HashMap::new();
    let mut grid_part_2: HashMap<(u16, u16), u32> = HashMap::new();
    let mut lit_lights: u32 = 0;
    let mut total_brightness: u32 = 0;

    for line in input.lines() {
        let instruction: &str = if line.starts_with("toggle") {
            "toggle"
        } else if line.starts_with("turn on") {
            "turn on"
        } else if line.starts_with("turn off") {
            "turn off"
        } else {
            continue;
        };

        let coords: Vec<u16> = line
            .split(|c: char| !c.is_numeric())
            .filter_map(|s| s.parse().ok())
            .collect();

        if coords.len() != 4 {
            println!("[!] Invalid line: {}", line);
            continue;
        }

        modify_grid_part_1(&mut grid_part_1, instruction, coords.clone());
        modify_grid_part_2(&mut grid_part_2, instruction, coords);
    }

    for &state in grid_part_1.values() {
        if state {
            lit_lights += 1;
        }
    }

    for &brightness in grid_part_2.values() {
        total_brightness += brightness;
    }

    (lit_lights, total_brightness)
}

fn modify_grid_part_1(grid: &mut HashMap<(u16, u16), bool>, instruction: &str, coords: Vec<u16>) {
    let operation: fn(bool) -> bool = match instruction {
        "turn on" => |_| true,
        "turn off" => |_| false,
        "toggle" => |state: bool| !state,
        _ => return,
    };

    for x in coords[0]..=coords[2] {
        for y in coords[1]..=coords[3] {
            let entry: &mut bool = grid.entry((x, y)).or_insert(false);
            *entry = operation(*entry);
        }
    }
}

fn modify_grid_part_2(grid: &mut HashMap<(u16, u16), u32>, instruction: &str, coords: Vec<u16>) {
    let operation: fn(u32) -> u32 = match instruction {
        "turn on" => |brightness: u32| brightness + 1,
        "turn off" => |brightness: u32| brightness.saturating_sub(1),
        "toggle" => |brightness: u32| brightness + 2,
        _ => return,
    };

    for x in coords[0]..=coords[2] {
        for y in coords[1]..=coords[3] {
            let entry: &mut u32 = grid.entry((x, y)).or_insert(0);
            *entry = operation(*entry);
        }
    }
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
