use std::collections::HashMap;
use std::env::args;
use std::fs;
use std::process;

fn puzzle(input: &str) {
    let mut houses_with_presents: u16 = 0;
    let mut presents: HashMap<String, u8> = HashMap::new();
    let mut robot_houses_with_presents: u16 = 0;
    let mut robot_presents: HashMap<String, u8> = HashMap::new();
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
            println!("Character: '{}' is not valid", char);
        }

        let coordinate: String = format!("{}, {}", x, y);
        *presents.entry(coordinate.to_owned()).or_default() += 1;

        if robot_turn {
            let coordinate: String = format!("{}, {}", robot_x, robot_y);
            *robot_presents.entry(coordinate.to_owned()).or_default() += 1;
        } else {
            let coordinate: String = format!("{}, {}", santa_x, santa_y);
            *robot_presents.entry(coordinate.to_owned()).or_default() += 1;
        }

        robot_turn = !robot_turn
    }

    for (_key, _value) in presents.iter() {
        houses_with_presents += 1;
    }
    for (_key, _value) in robot_presents.iter() {
        robot_houses_with_presents += 1;
    }

    println!("[+] Part 1: {}", houses_with_presents);
    println!("[+] Part 2: {}", robot_houses_with_presents);
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

    puzzle(&input);
}
