use std::env::args;
use std::fs::read_to_string;
use std::process::exit;

fn puzzle(input: &str) -> (String, String) {
    let mut code_part_1: String = String::new();
    let mut code_part_2: String = String::new();
    let mut x_part_1: i8 = 0;
    let mut x_part_2: i8 = -2;
    let mut x_values_part_1: Vec<i8> = Vec::new();
    let mut x_values_part_2: Vec<i8> = Vec::new();
    let mut y_part_1: i8 = 0;
    let mut y_part_2: i8 = 0;
    let mut y_values_part_1: Vec<i8> = Vec::new();
    let mut y_values_part_2: Vec<i8> = Vec::new();

    for line in input.lines() {
        for char in line.chars() {
            match char {
                'U' => {
                    if y_part_1 < 1 {
                        y_part_1 += 1;
                    }
                    match x_part_2.abs() {
                        0 => {
                            if y_part_2 < 2 {
                                y_part_2 += 1;
                            }
                        }
                        1 => {
                            if y_part_2 < 1 {
                                y_part_2 += 1;
                            }
                        }
                        _ => {}
                    }
                }
                'D' => {
                    if y_part_1 > -1 {
                        y_part_1 -= 1;
                    }
                    match x_part_2.abs() {
                        0 => {
                            if y_part_2 > -2 {
                                y_part_2 -= 1;
                            }
                        }
                        1 => {
                            if y_part_2 > -1 {
                                y_part_2 -= 1;
                            }
                        }
                        _ => {}
                    }
                }
                'L' => {
                    if x_part_1 > -1 {
                        x_part_1 -= 1;
                    }
                    match y_part_2.abs() {
                        0 => {
                            if x_part_2 > -2 {
                                x_part_2 -= 1;
                            }
                        }
                        1 => {
                            if x_part_2 > -1 {
                                x_part_2 -= 1;
                            }
                        }
                        _ => {}
                    }
                }
                'R' => {
                    if x_part_1 < 1 {
                        x_part_1 += 1;
                    }
                    match y_part_2.abs() {
                        0 => {
                            if x_part_2 < 2 {
                                x_part_2 += 1;
                            }
                        }
                        1 => {
                            if x_part_2 < 1 {
                                x_part_2 += 1;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {
                    println!("[!] Invalid character: {}", char);
                    continue;
                }
            }
        }

        x_values_part_1.push(x_part_1);
        x_values_part_2.push(x_part_2);
        y_values_part_1.push(y_part_1);
        y_values_part_2.push(y_part_2);
    }

    for (x, y) in x_values_part_1.iter().zip(&y_values_part_1) {
        let key = get_key_part_1(*x, *y);
        code_part_1.push(key);
    }

    for (x, y) in x_values_part_2.iter().zip(&y_values_part_2) {
        let key = get_key_part_2(*x, *y);
        code_part_2.push(key);
    }

    (code_part_1, code_part_2)
}

fn get_key_part_1(x: i8, y: i8) -> char {
    let keypad: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

    let index_row: usize = match y {
        -1 => 2,
        0 => 1,
        1 => 0,
        _ => panic!("[!] Invalid y coordinate: {}", y),
    };

    let index_col: usize = match x {
        -1 => 0,
        0 => 1,
        1 => 2,
        _ => panic!("[!] Invalid x coordinate: {}", x),
    };

    keypad[index_row][index_col]
}

fn get_key_part_2(x: i8, y: i8) -> char {
    let keypad: [[char; 5]; 5] = [
        ['0', '0', '1', '0', '0'],
        ['0', '2', '3', '4', '0'],
        ['5', '6', '7', '8', '9'],
        ['0', 'A', 'B', 'C', '0'],
        ['0', '0', 'D', '0', '0'],
    ];

    let index_row: usize = match y {
        -2 => 4,
        -1 => 3,
        0 => 2,
        1 => 1,
        2 => 0,
        _ => panic!("[!] Invalid y coordinate: {}", y),
    };

    let index_col: usize = match x {
        -2 => 0,
        -1 => 1,
        0 => 2,
        1 => 3,
        2 => 4,
        _ => panic!("[!] Invalid x coordinate: {}", x),
    };

    keypad[index_row][index_col]
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
