use std::env::args;
use std::fs::read_to_string;
use std::process::exit;

fn puzzle(input: &str) -> (String, String) {
    let mut code_part_1: String = String::new();
    let mut code_part_2: String = String::new();
    let mut x_part_1: i8 = 0;
    let mut x_part_2: i8 = -2;
    let mut y_part_1: i8 = 0;
    let mut y_part_2: i8 = 0;

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

        code_part_1.push(get_key_part_1(x_part_1, y_part_1));
        code_part_2.push(get_key_part_2(x_part_2, y_part_2));
    }

    (code_part_1, code_part_2)
}

fn get_key_part_1(x: i8, y: i8) -> char {
    #[rustfmt::skip]
    let keypad: [[char; 3]; 3] = [
        ['1', '2', '3'],
        ['4', '5', '6'],
        ['7', '8', '9'],
    ];

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
        ['.', '.', '1', '.', '.'],
        ['.', '2', '3', '4', '.'],
        ['5', '6', '7', '8', '9'],
        ['.', 'A', 'B', 'C', '.'],
        ['.', '.', 'D', '.', '.'],
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
