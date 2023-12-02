use std::{fs::{self}, collections::HashMap, str::Lines};

fn main() {
    run_day_1();
}

fn run_day_1() {
    let file_path = "data/day-1.txt";
    // let file_path = "data/day-1-2-sample.txt";
    let file_data = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    let input = file_data.lines();
    
    println!("Day 1-1 answer: {}", get_day_1_1_answer(input.clone()));
    println!("Day 1-2 answer: {}", get_day_1_2_answer(input.clone()));
}

fn get_day_1_1_answer(input_data: Lines) -> String {
    let numbered_values = input_data.map(|x| get_all_numbers_from_string(x, false));
    let calibration_values = numbered_values.map(|x| get_calibration(x));
    let total: i32 = calibration_values.sum();
    assert_eq!(total, 53921);
    return total.to_string();
}

fn get_day_1_2_answer(input_data: Lines) -> String {
    let numbered_values = input_data.map(|x| get_all_numbers_from_string(x, true));

    let calibration_values = numbered_values.map(|x| get_calibration(x));
    let total: i32 = calibration_values.sum();
    assert_eq!(total, 54676); 
    return total.to_string();
}

/// Returns the first and last digit in the vector concatenated as a two digit integer 
fn get_calibration(values: Vec<u32>) -> i32 {
    // Combine first and last number
    let str_number: String = format!("{}{}", values.first().unwrap(), values.last().unwrap());
    // Parse into a string
    return str_number.parse().unwrap()
}

/// Retrieves all numbers from strings. Spelt out numbers between one and nine will be interpreted as numbers and converted.
fn get_all_numbers_from_string(value: &str, parse_spelt_numbers: bool) -> Vec<u32> {
    let replacement_map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut all_numbers: Vec<u32> = Vec::new();
    let mut characters_since_match: String = String::new();
    // Loop over all characters
    for current_char in value.chars() {
        // Check if current is a numeric character
        if current_char.is_numeric() {
            all_numbers.push(char::to_digit(current_char, 10).unwrap());
            // A match has been found, clear characters since last match
            characters_since_match = String::from("");
            continue;
        }

        if !parse_spelt_numbers {
            continue;
        }

        // No match found, add current character to check for spelt number
        characters_since_match.push(current_char);
        for spelt_number in replacement_map.keys() {
            if characters_since_match.ends_with(spelt_number) {
                //Spelt out number found, convert to int via replacement map
                let int_number = replacement_map.get(spelt_number).unwrap();
                all_numbers.push(int_number.clone());
            }
        }
    }
    return all_numbers;
}
