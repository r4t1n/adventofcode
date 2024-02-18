use std::collections::HashSet;
use std::env::args;
use std::fs::read_to_string;
use std::process::exit;

fn puzzle(input: &str) -> (u16, u16) {
    let mut presents_part_1: HashSet<(i16, i16)> = HashSet::new();
    let mut presents_part_2: HashSet<(i16, i16)> = HashSet::new();
    let mut robot_turn: bool = false;
    let mut robot_x: i16 = 0;
    let mut robot_y: i16 = 0;
    let mut santa_x: i16 = 0;
    let mut santa_y: i16 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;

    for char in input.chars() {
        match char {
            '^' => {
                y += 1;
                if robot_turn {
                    robot_y += 1;
                } else {
                    santa_y += 1;
                }
            }
            'v' => {
                y -= 1;
                if robot_turn {
                    robot_y -= 1;
                } else {
                    santa_y -= 1;
                }
            }
            '>' => {
                x += 1;
                if robot_turn {
                    robot_x += 1;
                } else {
                    santa_x += 1;
                }
            }
            '<' => {
                x -= 1;
                if robot_turn {
                    robot_x -= 1;
                } else {
                    santa_x -= 1;
                }
            }
            _ => {
                println!("[!] Invalid character: {}", char);
                continue;
            }
        }

        presents_part_1.insert((x, y));

        if robot_turn {
            presents_part_2.insert((robot_x, robot_y));
        } else {
            presents_part_2.insert((santa_x, santa_y));
        }

        robot_turn = !robot_turn
    }

    let houses_with_presents_part_1: u16 = presents_part_1.len() as u16;
    let houses_with_presents_part_2: u16 = presents_part_2.len() as u16;

    (houses_with_presents_part_1, houses_with_presents_part_2)
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("[!] Usage: {} <input file>", args[0]);
        exit(1);
    }

    let input_filename: &String = &args[1];
    let input: String = match read_to_string(input_filename) {
        Ok(input) => input.trim().to_string(),
        Err(err) => {
            eprintln!("[!] Error reading file: {}", err);
            exit(1);
        }
    };

    let (answer_part_1, answer_part_2) = puzzle(&input);

    println!("[+] Part 1: {}", answer_part_1);
    println!("[+] Part 2: {}", answer_part_2);
}
