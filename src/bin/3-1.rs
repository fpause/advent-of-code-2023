use regex::Regex;
use uuid::Uuid;

use advent_of_code_2023::read_resource_lines;

#[derive(Debug)]
struct EnginePart {
    number: i32,
    id: Uuid,
    positions: Vec<usize>,
}

impl EnginePart {
    fn is_any_in_range(&self, indices: &Vec<usize>) -> bool {
        return indices.iter().any(|e| self.positions.contains(e));
    }
}

fn main() {
    let lines = read_resource_lines("3.txt");

    let (mut previous_symbols, previous_digits) = get_symbols_and_digits_for_line(&lines, 0);
    let (mut current_symbols, mut current_digits) = get_symbols_and_digits_for_line(&lines, 1);
    let (mut next_symbols, _) = get_symbols_and_digits_for_line(&lines, 2);
    let mut next_digits: Vec<EnginePart> = Vec::new();

    let mut sum = 0;
    let mut discovered_parts: Vec<Uuid> = Vec::new();

    sum = accumulate_digit_if_adjacent_symbol(&previous_symbols, &previous_digits, &None, &Some(&current_symbols), sum, &mut discovered_parts);

    for index in 1..lines.len() - 1 {
        (previous_symbols, _) = get_symbols_and_digits_for_line(&lines, index - 1);
        (current_symbols, current_digits) = get_symbols_and_digits_for_line(&lines, index);
        (next_symbols, next_digits) = get_symbols_and_digits_for_line(&lines, index + 1);

        sum = accumulate_digit_if_adjacent_symbol(&current_symbols, &current_digits, &Some(&previous_symbols), &Some(&next_symbols), sum, &mut discovered_parts);
    }

    sum = accumulate_digit_if_adjacent_symbol(&next_symbols, &next_digits, &Some(&current_symbols), &None, sum, &mut discovered_parts);

    println!("Result: {}", sum);
}

fn accumulate_digit_if_adjacent_symbol(current_symbols: &Vec<usize>, current_digits: &Vec<EnginePart>, previous_symbols: &Option<&Vec<usize>>, next_symbols: &Option<&Vec<usize>>, sum: i32, discovered_parts: &mut Vec<Uuid>) -> i32 {
    let mut tmp_sum = sum;
    let undiscovered_digits: Vec<&EnginePart> = current_digits.iter().filter(|e| !discovered_parts.contains(&e.id)).collect();
    for digit in undiscovered_digits {
        if (previous_symbols.is_some() && digit.is_any_in_range(previous_symbols.unwrap())) || digit.is_any_in_range(current_symbols) || (next_symbols.is_some() && digit.is_any_in_range(next_symbols.unwrap())) {
            tmp_sum += digit.number;
            discovered_parts.push(digit.id);
        }
    }
    tmp_sum
}

fn get_symbols_and_digits_for_line(lines: &Vec<String>, index: usize) -> (Vec<usize>, Vec<EnginePart>) {
    let current_line = &lines[index];
    (get_symbol_indices(current_line), get_digit_indices_and_lengths(current_line))
}

fn get_symbol_indices(line: &String) -> Vec<usize> {
    let symbols_re = Regex::new(r"([^0-9.])").expect("Problem with Regex");

    let mut symbol_indices: Vec<usize> = symbols_re.captures_iter(line)
        .map(|e| e.get(0).unwrap().start())
        .map(|e| vec![e - 1, e, e + 1])
        .flatten()
        .collect();
    symbol_indices.sort();
    symbol_indices.dedup();
    symbol_indices
}

fn get_digit_indices_and_lengths(line: &String) -> Vec<EnginePart> {
    let digits_re = Regex::new(r"([0-9]+)").expect("Problem with Regex");
    let digit_indices_and_lengths: Vec<EnginePart> = digits_re.captures_iter(line)
        .map(|c| (c.get(0).unwrap().start(), c.get(0).unwrap().len(), c.get(0).unwrap().as_str()))
        .map(|c| EnginePart { number: c.2.parse::<i32>().unwrap(), id: Uuid::new_v4(), positions: expand_indices(c.0, c.1.clone()) })
        .collect();

    digit_indices_and_lengths
}

fn expand_indices(index: usize, length: usize) -> Vec<usize> {
    let mut current_index = index;
    let mut res: Vec<usize> = Vec::new();
    for _ in 1..length + 1 {
        res.push(current_index);
        current_index += 1;
    }
    res
}