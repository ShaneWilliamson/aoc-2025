use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut zero_count = 0;
    let mut cursor = 50;

    let file = File::open("test_input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut s = line?;

        let first = match s.chars().next() {
            Some(c) => c,
            _ => continue, // skip this line as the format is not right
        };
        s = s.chars().skip(1).collect();
        let maybe_click = s.parse::<i32>();
        let click_num = match maybe_click {
            Err(_) => continue,
            Ok(num) => num,
        };
        cursor += match parse_directional_clicked(&first, click_num) {
            Ok(value) => value,
            Err(_) => {
                println!("ERROR: invalid direction!");
                continue;
            }
        };
        cursor = bind_range(&cursor);
        if cursor == 0 {
            zero_count += 1;
        }
    }

    println!(
        "The number of zero-results (actual password) is {}",
        zero_count
    );

    Ok(())
}

fn parse_directional_clicked(direction: &char, clicks: i32) -> Result<i32, ()> {
    match direction {
        'L' => Ok(-clicks),
        'R' => Ok(clicks),
        _ => Err(()),
    }
}

fn bind_range(cursor: &i32) -> i32 {
    cursor % 100
}
