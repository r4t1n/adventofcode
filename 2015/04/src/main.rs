use md5::Digest;
use std::env::args;
use std::fs;
use std::process;

fn puzzle(input: &str) {
    let mut nonce_part_1: u32 = 0;
    let mut nonce_part_2: u32 = 0;
    let prefix_part_1: &str = "00000";
    let prefix_part_2: &str = "000000";

    loop {
        let combined_input: String = format!("{}{}", input, nonce_part_1);
        let hash: Digest = md5::compute(combined_input);
        if format!("{:?}", hash).starts_with(prefix_part_1) {
            break;
        }

        nonce_part_1 += 1;
    }

    loop {
        let combined_input: String = format!("{}{}", input, nonce_part_2);
        let hash: Digest = md5::compute(combined_input);

        if format!("{:?}", hash).starts_with(prefix_part_2) {
            break;
        }

        nonce_part_2 += 1;
    }

    println!("[+] Part 1: {}", nonce_part_1);
    println!("[+] Part 2: {}", nonce_part_2);
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
