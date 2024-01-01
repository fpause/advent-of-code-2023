use regex::Regex;

use advent_of_code_2023::read_resource_lines;

fn main() {
    let lines = read_resource_lines("1.txt");
    let find_digit_regex = Regex::new(r"(\d)").unwrap();

    let mut vec: Vec<u32> = Vec::new();
    for line in lines {
        let number_string = extract_first_and_last_number(&find_digit_regex, &line);
        vec.push(number_string.parse().unwrap());
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

fn extract_first_and_last_number(find_digit_regex: &Regex, line: &String) -> String {
    let mut caps = find_digit_regex.captures_iter(&line).map(|c| c.extract());
    let (_, [first_digit]) = caps.next().unwrap();
    let last = caps.last();
    let mut number_string: String = first_digit.to_owned();
    match last {
        Some(number) => {
            let (_, [last_digit]) = number;
            number_string.push_str(last_digit);
        }
        None => {
            number_string = number_string.repeat(2);
        }
    }
    number_string
}
