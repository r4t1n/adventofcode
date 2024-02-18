use md5::Digest;
use std::env::args;
use std::fs::read_to_string;
use std::process::exit;

fn puzzle(input: &str) -> (u32, u32) {
    let prefix_part_1: &str = "00000";
    let prefix_part_2: &str = "000000";
    let nonce_part_1: u32 = compute_nonce(input, prefix_part_1);
    let nonce_part_2: u32 = compute_nonce(input, prefix_part_2);

    (nonce_part_1, nonce_part_2)
}

fn compute_nonce(input: &str, prefix: &str) -> u32 {
    let mut nonce: u32 = 0;

    loop {
        let input_with_nonce: String = format!("{}{}", input, nonce);
        let checksum: Digest = md5::compute(input_with_nonce);
        if format!("{:?}", checksum).starts_with(prefix) {
            break;
        }

        nonce += 1;
    }

    nonce
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
