use std::{fs::{self}, collections::HashMap, str::Lines};

use regex::Regex;

fn main() {
    run_day_2();
    run_day_1();
}

fn run_day_2() {
    let file_path = "data/day-2.txt";
    let file_data = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    let input = file_data.lines();
    
    println!("Day 2-1 answer: {}", get_day_2_1_answer(input.clone()));
}

fn get_day_2_1_answer(input_data: Lines) -> usize {
    let dice_data = input_data.map(|x| convert_dice_data(x));
    let possible_games = dice_data.enumerate().filter(|x| possible_dice_game(x.1, 12, 13, 14));
    for x in possible_games.clone() {
        println!("Line {} is possible", x.0);
    }
    let possible_games_ids = possible_games.map(|x| x.0 + 1);
    let total = possible_games_ids.sum();
    return total;    
}
// Converts a line of game data into a vector representing the maximum possible red, green, and blue dice respectively.
fn convert_dice_data(input: &str) -> (u32, u32, u32) {
    // Strip out the game number
    let game_data = input.split(':').skip(1).next().unwrap();
    // Split into individual dice pulls
    let dice_pulls = game_data.split(';');
    let regex = Regex::new(r"(\d+) (green|red|blue)").unwrap();

    let mut red = Vec::new();
    let mut green = Vec::new();
    let mut blue = Vec::new();

    // Loop over each dice pull
    for game in dice_pulls {
        let matches = regex.captures_iter(game);
        // Add the number of dice to the appropriate colour vector
        for colour_group in matches {
            let (_, [str_number, colour]) = colour_group.extract();
            let number: u32 = str_number.parse().unwrap();
            match colour {
                "red" => red.push(number),
                "green" => green.push(number),
                "blue" => blue.push(number),
                _ => panic!("Unknown colour found")
            }
        }
    }
    let max_red = red.iter().max().unwrap();
    let max_green = green.iter().max().unwrap();
    let max_blue = blue.iter().max().unwrap();
    return (*max_red, *max_green, *max_blue);
}

/// Checks if a given game exceeds the maximum number of dice provided.
fn possible_dice_game(game: (u32, u32, u32), max_red: u32, max_green: u32, max_blue: u32) -> bool{
    let total_dice = game.0 + game.1 + game.2;
    let max_dice = max_red + max_green + max_blue;
    if total_dice > max_dice{
        return false;
    }
    else if game.0 <= max_red && game.1 <= max_green && game.2 <= max_blue {
        return true;
    } else {
        return false;
    }
}

fn run_day_1() {
    let file_path = "data/day-1.txt";
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
