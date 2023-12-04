use std::{fs::{self}, collections::HashMap, str::Lines};

use regex::Regex;

struct DicePull {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    run_day_3();
    run_day_2();
    run_day_1();
}

fn run_day_3() {
    let file_path = "data/day-3.txt";
    let file_data = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    let input = file_data.lines();
    
    println!("Day 3-1 answer: {}", get_day_3_1_answer(input.clone()));
}

fn get_day_3_1_answer(input_data: Lines) -> usize {
    let engine_matrix: Vec<Vec<char>> = input_data.map(|x| convert_engine_data(x)).collect();
    let part_numbers = find_part_numbers(&engine_matrix);
    // assert_eq!(total, 2101);
    let total = part_numbers.iter().sum();
    return total;    
}

fn convert_engine_data(input: &str) -> Vec<char> {
    return input.chars().collect();
}

fn find_part_numbers(matrix: &Vec<Vec<char>>) -> Vec<usize> {
    let mut part_numbers = Vec::new();
    // Check each line
    for (line_number, line) in matrix.iter().enumerate() {
        // Variable to hold consecutive numbers
        let mut current_number = String::new();
        // Iterate over current line
        for (character_number, character) in line.iter().enumerate() {
            if character.is_numeric() {
                // Add found number to current list
                current_number.push(*character);

            }
            // If the character isn't numeric or we're at the end of the line.
            if !character.is_numeric() || character_number == line.len() - 1 {
                // Check if we've found one or more numbers
                if !current_number.is_empty() {
                    // Determine the start of the character
                    let mut start_of_char = character_number - current_number.len();
                    // If it's the end of the line, need to add one extra
                    if character.is_numeric() && character_number == line.len() - 1 {
                        start_of_char += 1;
                    } 
                    // Check if adjacent to a symbol and add it to the list if so.
                    if is_number_adjacent_to_symbol(start_of_char, line_number, current_number.len(), &matrix) {
                        // println!("Valid number: {}", current_number);
                        part_numbers.push(current_number.parse().unwrap());
                    }
                    // Clear used number regardless of it was added to list
                    current_number.clear()
                }
            }
        }
    }
    
    return part_numbers;
}

fn is_number_adjacent_to_symbol(x: usize, y: usize, length: usize, matrix: &Vec<Vec<char>>) -> bool{
    let min_x = if x == 0 { 0 } else { x - 1 };
    let max_x = (x + length).min(matrix[0].len() - 1);
    let mut match_found = false;
    // Debug output
    // println!("({},{})", min_x + 1, y);

    // Check previous row (if not first row)
    if y > 0 {
        let previous_row = y - 1;
        for x_pos in min_x..=max_x {
            let current_char = matrix[previous_row][x_pos];
            // print!("{}", current_char);
            if is_special_character(current_char) {
                match_found = true;
            }
        }
    }
    println!();

    // Check same row
    // for x_pos in min_x..=max_x {
    //     print!("{}", matrix[y][x_pos]);
    // }
    if is_special_character(matrix[y][min_x]) || is_special_character(matrix[y][max_x]) {
        match_found = true;
    }
    println!();

    // Check next row (if not last)
    if y != matrix.len() - 1 {
        let next_row = y + 1;
        for x_pos in min_x..=max_x {
            let current_char = matrix[next_row][x_pos];
            // print!("{}", current_char);
            if is_special_character(current_char) {
                match_found = true;
            }
        }
        // println!();
    }

    // println!("Match: {}", match_found);
    // println!();
    return match_found;
}

fn is_special_character(character: char) -> bool{
    return !character.is_alphanumeric() && character != '.';
}

fn run_day_2() {
    let file_path = "data/day-2.txt";
    let file_data = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    let input = file_data.lines();
    
    println!("Day 2-1 answer: {}", get_day_2_1_answer(input.clone()));
    println!("Day 2-2 answer: {}", get_day_2_2_answer(input.clone()));
}

fn get_day_2_1_answer(input_data: Lines) -> usize {
    let dice_data = input_data.map(|x| convert_dice_data(x));
    let max_dice_data = dice_data.map(get_minimum_dice_possible_from_game);
    let possible_games = max_dice_data.enumerate().filter(|x| possible_dice_game(&x.1, 12, 13, 14));

    let possible_games_ids = possible_games.map(|x| x.0 + 1);
    let total = possible_games_ids.sum();
    // assert_eq!(total, 2101);
    return total;    
}

fn get_day_2_2_answer(input_data: Lines) -> u32 {
    let dice_data = input_data.map(|x| convert_dice_data(x));
    let min_dice_data = dice_data.map(get_minimum_dice_possible_from_game);
    let dice_power = min_dice_data.map(get_dice_power);

    let total = dice_power.sum();
    assert_eq!(total, 58269);
    return total;
}

// Converts a line of game data into a vector representing the individual dice pulls
fn convert_dice_data(input: &str) -> Vec<DicePull> {
    // Strip out the game number
    let game_data = input.split(':').skip(1).next().unwrap();
    // Split into individual dice pulls
    let dice_pulls = game_data.split(';');
    let regex = Regex::new(r"(\d+) (green|red|blue)").unwrap();

    let mut games: Vec<DicePull> = Vec::new();

    // Loop over each dice pull
    for game_data in dice_pulls {
        let mut current_game = DicePull { red: 0, green: 0, blue: 0 };
        let matches = regex.captures_iter(game_data);
        // Add the number of dice to the appropriate colour vector
        for colour_group in matches {
            let (_, [str_number, colour]) = colour_group.extract();
            let number: u32 = str_number.parse().unwrap();
            match colour {
                "red" => current_game.red = number,
                "green" => current_game.green = number,
                "blue" => current_game.blue = number,
                _ => panic!("Unknown colour found")
            }
        
        }
        games.push(current_game);
    }
    return games;
}

/// Determine the minimum possible number of each dice from multiple dice pulls
fn get_minimum_dice_possible_from_game(games: Vec<DicePull>) -> DicePull {
    let mut max_possible_dice = DicePull { red: 0, green: 0, blue: 0 };
    for game in games {
        if game.red > max_possible_dice. red {
            max_possible_dice.red = game.red;
        }
        if game.green > max_possible_dice. green {
            max_possible_dice.green = game.green;
        }
        if game.blue > max_possible_dice. blue {
            max_possible_dice.blue = game.blue;
        }
    }
    return max_possible_dice;
}

/// Checks if a given game exceeds the maximum number of dice provided.
fn possible_dice_game(game: &DicePull, max_red: u32, max_green: u32, max_blue: u32) -> bool{
    let total_dice = game.red + game.green + game.blue;
    let max_dice = max_red + max_green + max_blue;
    if total_dice > max_dice{
        return false;
    }
    else if game.red <= max_red && game.green <= max_green && game.blue <= max_blue {
        return true;
    } else {
        return false;
    }
}

/// Determines the minimum "power" of a set of dice by multiplying the known number of each dice.
fn get_dice_power(dice_pull: DicePull) -> u32 {
    return dice_pull.red * dice_pull.green * dice_pull.blue;
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
