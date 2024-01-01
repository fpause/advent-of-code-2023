use std::collections::HashMap;

use regex::Regex;

use advent_of_code_2023::read_resource_lines;

#[derive(Debug)]
struct DigitMatch<'a> {
    index: usize,
    value: &'a str,
}

fn main() {
    let lines = read_resource_lines("1.txt");

    let mut vec: Vec<u32> = Vec::new();
    for line in lines {
        let number_string = extract_first_and_last_number(&line);
        if let Ok(number) = number_string.parse() {
            vec.push(number);
        } else {
            println!("Error parsing line: {}", line);
        }
    }
    let result = sum_and_print(&mut vec);
    println!("Result: {}", result);
}

fn sum_and_print(vec: &mut Vec<u32>) -> u32 {
    let mut result = 0u32;
    for number in vec.iter() {
        result += number;
    }
    result
}

fn extract_first_and_last_number(line: &String) -> String {
    let find_digit_regex = Regex::new(r"(\d)").unwrap();
    let find_word_digit_regexes: Vec<Regex> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter().map(|number| Regex::new(&*format!("({})", number)).unwrap()).collect();
    let mapping = HashMap::from([("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")]);

    let mut all_matches: Vec<DigitMatch> = Vec::new();

    for regex in find_word_digit_regexes {
        for capture in regex.captures_iter(&line) {
            if let Some(value) = capture.get(0) {
                all_matches.push(DigitMatch { index: value.start(), value: mapping.get(value.as_str()).unwrap() })
            }
        }
    }
    for capture in find_digit_regex.captures_iter(&line) {
        if let Some(value) = capture.get(0) {
            all_matches.push(DigitMatch { index: value.start(), value: value.as_str() })
        }
    }

    let mut number_string = String::new();
    all_matches.sort_by(|m1, m2| m1.index.cmp(&m2.index));
    let first_digit = all_matches.get(0).unwrap();
    let last_digit = all_matches.get(all_matches.len() - 1).unwrap();
    number_string.push_str(first_digit.value);
    number_string.push_str(last_digit.value);
    number_string
}