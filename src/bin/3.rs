use regex::Regex;

use advent_of_code_2023::read_resource_lines;

#[derive(Debug)]
struct EnginePart {
    number: i32,
    positions: Vec<usize>,
}

impl EnginePart {
    fn is_in_range(&self, index: usize) -> bool {
        self.positions.contains(&index)
    }
}

fn main() {
    let lines = read_resource_lines("3.txt");
    let mut count = 0;
    let first_line = lines.first().unwrap();

    let mut previous_line_digits = get_digit_indices_and_lengths(first_line);

    for index in 2..lines.len() - 1 {
        get_symbols_and_digits_for_line(&lines, index);

        count += 1;
        if count == 3 {
            break;
        }
    }
}

fn get_symbols_and_digits_for_line(lines: &Vec<String>, index: usize) -> (Vec<usize>, Vec<EnginePart>) {
    let current_line = &lines[index];
    (get_symbol_indices(current_line), get_digit_indices_and_lengths(current_line))
}

fn get_symbol_indices<T: AsRef<str>>(line: &[T]) -> Vec<usize> {
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

fn get_digit_indices_and_lengths<T: AsRef<str>>(line: &[T]) -> Vec<EnginePart> {
    let digits_re = Regex::new(r"([0-9]+)").expect("Problem with Regex");
    let digit_indices_and_lengths: Vec<EnginePart> = digits_re.captures_iter(line)
        .map(|c| (c.get(0).unwrap().start(), c.get(0).unwrap().len(), c.get(0).unwrap().as_str()))
        .map(|c| EnginePart { number: c.2.parse::<i32>().unwrap(), positions: expand_indices(c.0, c.1.clone()) })
        .collect();
    digit_indices_and_lengths
}

fn expand_indices(index: usize, length: usize) -> Vec<usize> {
    let mut current_index = index;
    let mut res: Vec<usize> = Vec::new();
    for i in 1..length + 1 {
        res.push(current_index);
        current_index += 1;
    }
    res
}