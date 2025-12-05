use std::fs::File;
use std::io::{BufRead, BufReader};

// advent of code 2025 day 1 -> https://adventofcode.com/2025/day/1
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut zero_count = 0;
    let mut cursor = 50;

    let file = File::open("inputs/d1_test_input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut s = line?;

        let first = match s.chars().next() {
            Some(c) => c,
            _ => continue, // skip this line as the format is not right
        };
        s = s.chars().skip(1).collect();
        let click_num = match s.parse::<i64>() {
            Err(_) => continue,
            Ok(num) => num,
        };
        let next_cursor = cursor
            + match parse_directional_clicked(&first, click_num) {
                Ok(value) => value,
                Err(_) => {
                    println!("ERROR: invalid direction!");
                    continue;
                }
            };
        let bound_next_cursor = bind_range(&next_cursor);

        // we can calculate how many times it goes past 0 for times turned to the right and left
        // by integer division by 100, as well, if it drops down into negatives we increase one
        // time
        // case 1: cursor drops past 0 into negatives (did not start at 0) - passed 0 once
        if cursor > 0 && next_cursor < 0 {
            zero_count += 1;
        }
        // case 2: cursor does full turns going past zero in either direction
        if (next_cursor / 100).abs() > 0 {
            zero_count += (next_cursor / 100).abs();
            // edge case: it lands on a 0, we account for this already below
            if bound_next_cursor == 0 {
                zero_count -= 1;
            }
        }
        // case 3: cursor lands on 0
        if bound_next_cursor == 0 {
            zero_count += 1;
        }
        cursor = bound_next_cursor;
    }

    println!(
        "The number of zero-results (actual password) is {}",
        zero_count
    );

    Ok(())
}

fn parse_directional_clicked(direction: &char, clicks: i64) -> Result<i64, ()> {
    match direction {
        'L' => Ok(-clicks),
        'R' => Ok(clicks),
        _ => Err(()),
    }
}

fn bind_range(cursor: &i64) -> i64 {
    // apparently the '%' operator is a remainder operator, not modulo
    cursor.rem_euclid(100)
}
