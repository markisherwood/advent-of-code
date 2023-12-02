use std::{fs, collections::HashMap, str::Lines};

use regex::Regex;

fn main() {
    let file_path = "data/input-data.txt";
    // let file_path = "data/day-1-2-sample.txt";
    let file_data = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    let input = file_data.lines();
    
    //println!("Day 1-1 answer: {}", get_day_1_1_answer(input.clone()));
    println!("Day 1-2 answer: {}", get_day_1_2_answer(input.clone()));

}

fn get_day_1_1_answer(input_data: Lines) -> String {
    let calibration_values = input_data.map(|x| get_calibration(x));
    let total: i32 = calibration_values.sum();
    return total.to_string();
}

fn get_day_1_2_answer(input_data: Lines) -> String {
    let replaced_values = input_data.map(|x| replace_spelt_numbers_with_numeric(x));
    let calibration_values = replaced_values.map(|x| get_calibration(&x));
    let total: i32 = calibration_values.sum();
    return total.to_string();
}

fn get_calibration(value: &str) -> i32 {
    // Convert to char array
    let mut char: Vec<char> = value.chars().collect();
    // Drop all non numbers
    char.retain(|c| c.is_numeric());
    // Combine first and last number
    let str_number = format!("{}{}", char.first().unwrap(), char.last().unwrap());
    // Parse into a string
    println!("{}: {}", value, str_number);
    return str_number.parse().unwrap()
}

fn replace_spelt_numbers_with_numeric(value: &str) -> String {
    let mut replacement_map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    

    let mut new_value: String = value.to_string();
    let re = Regex::new(r"(one)|(two)|(three)|(four)|(fix)|(six)|(seven)|(eight)|(nine)").unwrap();
    for val in re.find_iter(value) {
        let find = val.as_str();
        let replace = replacement_map.get(find).unwrap();
        new_value = new_value.replace(find, replace);
    }
    println!("Old value: {}", value);
    println!("New value: {}", new_value);
    return new_value;
}
