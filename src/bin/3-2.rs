use regex::Regex;

use advent_of_code_2023::read_resource_lines;

#[derive(Debug)]
struct Gear<'a> {
    positions: Vec<usize>,
    engine_parts: Vec<&'a EnginePart>,
}

#[derive(Debug)]
struct EnginePart {
    number: usize,
    positions: Vec<usize>,
}

impl<'a> Gear<'a> {
    fn get_engine_parts_in_range(&self, engine_parts: &'a Vec<EnginePart>) -> Vec<&'a EnginePart> {
        let mut result: Vec<&'a EnginePart> = Vec::new();
        for engine_part in engine_parts {
            if engine_part.positions.iter().any(|e| self.positions.contains(e)) {
                result.push(engine_part)
            }
        }
        result
    }

    fn is_gear(&self) -> bool {
        return self.engine_parts.len() == 2;
    }

    fn get_gear_ratio(&self) -> i32 {
        return (self.engine_parts[0].number * self.engine_parts[1].number) as i32;
    }
}

fn main() {
    let lines = read_resource_lines("3.txt");

    let (mut previous_symbols, mut previous_digits) = get_symbols_and_digits_for_line(&lines, 0);
    let (_, mut current_digits) = get_symbols_and_digits_for_line(&lines, 1);
    let (mut next_symbols, _) = get_symbols_and_digits_for_line(&lines, 2);
    let mut current_symbols: Vec<Gear>;
    let mut next_digits: Vec<EnginePart> = Vec::new();

    let mut sum = 0;

    sum = accumulate_gear_ratios(&mut previous_symbols, &previous_digits, &None, &Some(&current_digits), sum);

    for index in 1..lines.len() - 1 {
        (_, previous_digits) = get_symbols_and_digits_for_line(&lines, index - 1);
        (current_symbols, current_digits) = get_symbols_and_digits_for_line(&lines, index);
        (next_symbols, next_digits) = get_symbols_and_digits_for_line(&lines, index + 1);

        sum = accumulate_gear_ratios(&mut current_symbols, &current_digits, &Some(&previous_digits), &Some(&next_digits), sum);
    }

    sum = accumulate_gear_ratios(&mut next_symbols, &next_digits, &Some(&current_digits), &None, sum);

    println!("Result: {}", sum);
}

fn accumulate_gear_ratios<'a>(current_gears: &mut Vec<Gear<'a>>, current_digits: &'a Vec<EnginePart>, previous_digits: &Option<&'a Vec<EnginePart>>, next_digits: &Option<&'a Vec<EnginePart>>, sum: i32) -> i32 {
    let mut tmp_sum = sum;
    for gear in current_gears {
        discover_adjacent_engine_parts(previous_digits, gear);
        discover_adjacent_engine_parts(&Some(current_digits), gear);
        discover_adjacent_engine_parts(next_digits, gear);
        if gear.is_gear() {
            tmp_sum += gear.get_gear_ratio();
        }
    }
    tmp_sum
}

fn discover_adjacent_engine_parts<'a>(previous_digits: &Option<&'a Vec<EnginePart>>, gear: &mut Gear<'a>) {
    if previous_digits.is_some() {
        let engine_parts = gear.get_engine_parts_in_range(previous_digits.unwrap());
        for engine_part in engine_parts {
            gear.engine_parts.push(&engine_part)
        }
    }
}

fn get_symbols_and_digits_for_line(lines: &Vec<String>, index: usize) -> (Vec<Gear>, Vec<EnginePart>) {
    let current_line = &lines[index];
    (get_gears(current_line), get_digit_indices_and_lengths(current_line))
}

fn get_gears(line: &String) -> Vec<Gear> {
    let symbols_re = Regex::new(r"(\*)").expect("Problem with Regex");

    let symbol_indices: Vec<Gear> = symbols_re.captures_iter(line)
        .map(|e| e.get(0).unwrap().start())
        .map(|e| vec![e - 1, e, e + 1])
        .map(|e| Gear { positions: e, engine_parts: Vec::new() })
        .collect();
    symbol_indices
}

fn get_digit_indices_and_lengths(line: &String) -> Vec<EnginePart> {
    let digits_re = Regex::new(r"([0-9]+)").expect("Problem with Regex");
    let digit_indices_and_lengths: Vec<EnginePart> = digits_re.captures_iter(line)
        .map(|c| (c.get(0).unwrap().start(), c.get(0).unwrap().len(), c.get(0).unwrap().as_str()))
        .map(|c| EnginePart { number: c.2.parse::<usize>().unwrap(), positions: expand_indices(c.0, c.1.clone()) })
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