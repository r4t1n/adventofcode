use md5::Digest;
use std::env::args;
use std::fs;
use std::process;

fn puzzle(input: &str) {
    let mut part_1_nonce: u32 = 0;
    let mut part_2_nonce: u32 = 0;
    let part_1_prefix: &str = "00000";
    let part_2_prefix: &str = "000000";

    loop {
        let combined_input: String = format!("{}{}", input, part_1_nonce);
        let hash: Digest = md5::compute(combined_input);
        if format!("{:?}", hash).starts_with(part_1_prefix) {
            break;
        }

        part_1_nonce += 1;
    }

    loop {
        let combined_input: String = format!("{}{}", input, part_2_nonce);
        let hash: Digest = md5::compute(combined_input);

        if format!("{:?}", hash).starts_with(part_2_prefix) {
            break;
        }

        part_2_nonce += 1;
    }

    println!("[+] Part 1: {}", part_1_nonce);
    println!("[+] Part 2: {}", part_2_nonce);
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
