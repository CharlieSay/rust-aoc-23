use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("./data/test_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;
    let digit_map = create_digit_map();

    for line in reader.lines() {
        let line = line?;
        let digits = extract_digits(&line, &digit_map);

        if digits.len() >= 2 {
            let first = digits[0];
            let last = digits[digits.len() - 1];
            let value = format!("{}{}", first, last).parse::<i32>().unwrap_or(0);
            sum += value;
        }
    }

    println!("Total sum of calibration values: {}", sum);

    Ok(())
}

fn create_digit_map() -> HashMap<String, char> {
    let mut map = HashMap::new();
    map.insert("zero".to_string(), '0');
    map.insert("one".to_string(), '1');
    map.insert("two".to_string(), '2');
    map.insert("three".to_string(), '3');
    map.insert("four".to_string(), '4');
    map.insert("five".to_string(), '5');
    map.insert("six".to_string(), '6');
    map.insert("seven".to_string(), '7');
    map.insert("eight".to_string(), '8');
    map.insert("nine".to_string(), '9');
    map
}

fn extract_digits(line: &str, digit_map: &HashMap<String, char>) -> Vec<char> {
    let mut result = Vec::new();
    let mut current_word = String::new();

    for ch in line.chars().chain([' '].iter().copied()) {
        if ch.is_alphabetic() {
            current_word.push(ch);
        } else {
            let lowercase_word = current_word.to_lowercase();
            if let Some(&digit) = digit_map.get(&lowercase_word) {
                result.push(digit);
                current_word.clear();
            }
            if ch.is_digit(10) {
                result.push(ch);
            }
        }
    }

    result
}
