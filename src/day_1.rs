use std;
use std::fs;

const SPECIAL_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn main() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let file_content: String = String::from_utf8_lossy(&fs::read("src/day_1.txt")?).parse()?;
    let mut sum: i32 = 0;
    for line in file_content.lines() {
        let mut first_number: i32 = -1;
        let mut second_number: i32 = -1;
        let mut found_first_number: bool = false;
        for (pos, char) in line.chars().enumerate() {
            let mut number = -1;
            if char.is_numeric() {
                number = char.to_digit(10).unwrap() as i32;
            } else {
                for (index, special_word) in SPECIAL_WORDS.iter().enumerate() {
                    if pos + special_word.len() > line.len() {
                        continue;
                    }
                    if *special_word == &line[pos..pos + special_word.len()] {
                        number = index as i32 + 1;
                        break;
                    }
                }
            }
            if number == -1 {
                continue;
            }

            if !found_first_number {
                first_number = number;
                found_first_number = true;
            }
            second_number = number;
        }

        if first_number == -1 || second_number == -1 {
            return Err("Could not find two numbers in line".into());
        }
        sum += first_number * 10 + second_number;
    }
    return Ok(sum.to_string());
}
