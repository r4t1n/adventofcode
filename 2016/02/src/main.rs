use std::env::args;
use std::fs;
use std::process;

fn puzzle(input: &str) -> (String, i32) {
    let answer_part_2: i32 = 0;
    let mut code: String = String::new();
    let mut values_x: Vec<i8> = Vec::new();
    let mut values_y: Vec<i8> = Vec::new();
    let mut x: i8 = 0;
    let mut y: i8 = 0;

    for line in input.lines() {
        for char in line.chars() {
            match char {
                'U' => {
                    if y < 1 {
                        y += 1;
                    }
                }
                'D' => {
                    if y > -1 {
                        y -= 1;
                    }
                }
                'L' => {
                    if x > -1 {
                        x -= 1;
                    }
                }
                'R' => {
                    if x < 1 {
                        x += 1;
                    }
                }
                _ => {
                    println!("[!] Invalid character: {}", char);
                    continue;
                }
            }
        }

        values_x.push(x);
        values_y.push(y);
    }

    for (x, y) in values_x.iter().zip(&values_y) {
        if let Some(key) = get_key(*x, *y) {
            code.push(key);
        } else {
            println!("[!] Invalid coordinate: {}, {}", x, y);
        }
    }

    (code, answer_part_2)
}

fn get_key(x: i8, y: i8) -> Option<char> {
    let keypad: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

    let index_row: usize = match y {
        -1 => 2,
        0 => 1,
        1 => 0,
        _ => return None,
    };

    let index_col: usize = match x {
        -1 => 0,
        0 => 1,
        1 => 2,
        _ => return None,
    };

    Some(keypad[index_row][index_col])
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("[!] Usage: {} <input file>", args[0]);
        process::exit(1);
    }

    let input_filename: &String = &args[1];
    let input: String = match fs::read_to_string(input_filename) {
        Ok(input) => input.trim().to_string(),
        Err(err) => {
            eprintln!("[!] Error reading file: {}", err);
            process::exit(1);
        }
    };

    let (answer_part_1, answer_part_2) = puzzle(&input);
    println!("[+] Part 1: {}", answer_part_1);
    println!("[+] Part 2: {}", answer_part_2);
}
