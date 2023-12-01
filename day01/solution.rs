use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn get_calibration_value(input: &str) -> u32 {
    let first_number = input.chars().find_map(|c| c.to_digit(10)).unwrap();
    let last_number = input.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
    first_number * 10 + last_number
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <input_file>", args[0]);
        println!("Example: {} input.txt", args[0]);
    } else {
        let input_file = &args[1];

        let file = File::open(input_file)?;
        let reader = BufReader::new(file);

        let answer = reader
            .lines()
            .map(|line| line.unwrap())
            .map(|line| get_calibration_value(&line))
            .sum::<u32>();
        println!("{}", answer);
    }
    Ok(())
}
