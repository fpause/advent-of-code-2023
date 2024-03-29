use std::cmp::min;
use std::collections::HashMap;

use advent_of_code_2023::read_resource_lines;

fn main() {
    let lines = read_resource_lines("4.txt");

    let mut scratch_lookup = HashMap::new();
    let mut scratch_result = HashMap::new();
    let mut index = 1;
    for line in lines {
        let matching_numbers = get_scratch_card_score(line);
        scratch_lookup.insert(index, matching_numbers);
        scratch_result.insert(index, 1);
        index += 1;
    }

    let mut total = 0;
    for index in 1..(scratch_result.len() + 1) as u32 {
        for idx in 1..min(scratch_lookup[&index] + 1, scratch_result.len() as u32 + 1 - index) {
            scratch_result.insert(index + idx, scratch_result[&(index + idx)] + scratch_result[&(index)]);
        }
        total += scratch_result[&(index)];
    }
    println!("Total score: {}", total);
}

fn get_scratch_card_score(line: String) -> u32 {
    let (game_numbers, my_numbers) = get_lists(line);
    let mut score = 0;
    for my_number in my_numbers {
        if game_numbers.contains(&my_number) {
            score += 1;
        }
    }
    score
}

fn get_lists(line: String) -> (Vec<i32>, Vec<i32>) {
    let mut split = line.split("|");
    let game_numbers = split.next().unwrap();
    let mut game_numbers_split = game_numbers.split(":");
    game_numbers_split.next();
    return (get_numbers_list(game_numbers_split.next().unwrap()), get_numbers_list(split.next().unwrap()));
}

fn get_numbers_list(numbers: &str) -> Vec<i32> {
    numbers.trim().split_whitespace().map(|e| e.trim().parse::<i32>().unwrap()).collect()
}