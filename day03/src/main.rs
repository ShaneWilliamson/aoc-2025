use std::fs::File;
use std::io::{BufRead, BufReader};

// advent of code 2025 day 3 -> https://adventofcode.com/2025/day/2
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("inputs/d3_test_input.txt")?;
    let reader = BufReader::new(file);

    let mut total_power = 0;

    for line in reader.lines() {
        let s = line?;
        let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();

        // initialize
        let mut tens_index = 0;

        // best is an o(n) solution
        for (idx, num) in digits[..digits.len() - 1].iter().enumerate() {
            if *num > digits[tens_index] {
                tens_index = idx;
            }
        }

        let tens = digits[tens_index];
        let mut ones = 0;

        // only check digits to the right of the tens digit
        for num in digits[tens_index + 1..digits.len()].iter() {
            if *num > ones {
                ones = *num;
            }
        }

        let bank_power = 10 * tens + ones;

        total_power += bank_power;
    }

    println!("The sum is {}", total_power);

    Ok(())
}
