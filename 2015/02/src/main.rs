use std::env::args;
use std::fs;
use std::process;

fn puzzle(input: &str) -> (u32, u32) {
    let mut ribbon_sum: u32 = 0;
    let mut wrapping_paper_sum: u32 = 0;

    for line in input.lines() {
        let mut dimensions: Vec<u8> = line
            .split('x')
            .filter_map(|s: &str| s.parse().ok())
            .collect();

        if dimensions.len() != 3 {
            println!("[!] Invalid line: {}", line);
            continue;
        }

        let (wrapping_paper, ribbon) = calculate_dimensions(&mut dimensions);
        wrapping_paper_sum += wrapping_paper as u32;
        ribbon_sum += ribbon as u32;
    }

    (wrapping_paper_sum, ribbon_sum)
}

fn calculate_dimensions(dimensions: &mut [u8]) -> (u16, u16) {
    let l: u16 = dimensions[0] as u16;
    let w: u16 = dimensions[1] as u16;
    let h: u16 = dimensions[2] as u16;

    let surface_area: u16 = (2 * l * w) + (2 * w * h) + (2 * h * l);
    let bow: u16 = l * w * h;

    dimensions.sort();
    let slack: u16 = dimensions[0] as u16 * dimensions[1] as u16;
    let wrapping_paper: u16 = surface_area + slack;
    let ribbon: u8 = dimensions[0] + dimensions[0] + dimensions[1] + dimensions[1];
    let total_ribbon: u16 = bow + ribbon as u16;

    (wrapping_paper, total_ribbon)
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
