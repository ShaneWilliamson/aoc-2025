use std::fs::File;
use std::io::{BufRead, BufReader};

// advent of code 2025 day 2 -> https://adventofcode.com/2025/day/2
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut invalid_id_sum = 0;

    let file = File::open("inputs/d2_test_input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let s = line?;

        let ranges = s.split(",");

        for rng in ranges {
            let mut id_start_end = rng.split("-");
            let maybe_id_start = id_start_end.next();
            let maybe_id_end = id_start_end.next();

            let id_start = match maybe_id_start {
                Some(id_start) => id_start,
                None => continue,
            };
            let id_end = match maybe_id_end {
                Some(id_end) => id_end,
                None => continue,
            };

            let invalid_ids = get_invalid_ids_from_range(id_start, id_end)?;

            for id in invalid_ids {
                invalid_id_sum += id.parse::<i64>()?;
            }
        }
    }

    println!("The sum is {}", invalid_id_sum);

    Ok(())
}

fn get_invalid_ids_from_range(
    id_start: &str,
    id_end: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // assume min decimals is 2
    let range_divisions = 2..=id_end.len();

    let start_num = id_start.parse::<i64>()?;
    let end_num = id_end.parse::<i64>()?;

    let range: Vec<i64> = (start_num..=end_num).collect();
    let mut invalid_ids = Vec::new();
    'outer: for id in range {
        'inner: for rd in range_divisions.clone() {
            let id_str = id.to_string();
            if id_str.len().is_multiple_of(rd) {
                let chars: Vec<char> = id_str.chars().collect();
                let chunks: Vec<&[char]> = chars.chunks(id_str.len() / rd).collect();
                for c in chunks.clone() {
                    // all need to be equal, so we can just check agaist the first
                    if c != chunks[0] {
                        continue 'inner;
                    }
                }
                // at this point we know it is invalid
                invalid_ids.push(id.to_string());
                continue 'outer;
            }
        }
    }

    Ok(invalid_ids)
}
