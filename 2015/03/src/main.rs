use std::collections::HashMap;
use std::env::args;
use std::fs;
use std::process;

fn puzzle(input: &str) -> (u16, u16) {
    let mut presents_part_1: HashMap<(i16, i16), u8> = HashMap::new();
    let mut presents_part_2: HashMap<(i16, i16), u8> = HashMap::new();
    let mut robot_turn: bool = false;
    let mut robot_x: i16 = 0;
    let mut robot_y: i16 = 0;
    let mut santa_x: i16 = 0;
    let mut santa_y: i16 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;

    for char in input.chars() {
        if char == '^' {
            y += 1;
            if robot_turn {
                robot_y += 1;
            } else {
                santa_y += 1;
            }
        } else if char == 'v' {
            y -= 1;
            if robot_turn {
                robot_y -= 1;
            } else {
                santa_y -= 1;
            }
        } else if char == '>' {
            x += 1;
            if robot_turn {
                robot_x += 1;
            } else {
                santa_x += 1;
            }
        } else if char == '<' {
            x -= 1;
            if robot_turn {
                robot_x -= 1;
            } else {
                santa_x -= 1;
            }
        } else {
            println!("[!] Invalid character: {}", char);
            continue;
        }

        *presents_part_1.entry((x, y).to_owned()).or_default() += 1;
        if robot_turn {
            *presents_part_2
                .entry((robot_x, robot_y).to_owned())
                .or_default() += 1;
        } else {
            *presents_part_2
                .entry((santa_x, santa_y).to_owned())
                .or_default() += 1;
        }

        robot_turn = !robot_turn
    }

    let houses_with_presents_part_1: u16 = presents_part_1.len().try_into().unwrap();
    let houses_with_presents_part_2: u16 = presents_part_2.len().try_into().unwrap();

    (houses_with_presents_part_1, houses_with_presents_part_2)
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
