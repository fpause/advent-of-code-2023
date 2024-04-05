use std::collections::HashMap;
use std::ops::Mul;

use num_bigint::BigUint;
use num_traits::ToPrimitive;
use prime_factorization::Factorization;

use advent_of_code_2023::read_resource_lines;

struct Crossing {
    left: String,
    right: String,
}

struct Scenario {
    instructions: Vec<char>,
    map: HashMap<String, Crossing>,
}

#[derive(Eq, PartialEq, Clone)]
struct PositionWithInstructionIndex {
    position: String,
    instruction_index: usize,
}

impl Scenario {
    fn read_from_file() -> Scenario {
        let lines = read_resource_lines("8.txt");
        let instructions = lines[0].chars().collect();
        let mut map = HashMap::new();
        for line in lines[2..].iter() {
            let condensed = line.replace(" ", "").replace("(", "").replace(")", ",").replace("=", ",");
            let split = condensed.split(",").collect::<Vec<&str>>();
            let key = String::from(split[0]);
            let left = String::from(split[1]);
            let right = String::from(split[2]);
            map.insert(key, Crossing { left, right });
        }
        Scenario {
            instructions,
            map,
        }
    }

    fn follow_instructions_till_zzz(&self) -> BigUint {
        let mut current_positions = self.get_all_starting_positions();

        println!("Starting positions: {:?}", current_positions);

        let mut cycle_lenghts = Vec::new();

        for &position in current_positions.iter() {
            let mut instruction_index = 0;
            let mut visited_positions = Vec::new();
            let mut current_position = PositionWithInstructionIndex {
                position: position.clone(),
                instruction_index: 0,
            };

            let mut index = 0u32;
            while !visited_positions.contains(&current_position) {
                visited_positions.push(current_position.clone());
                let crossing = self.map.get(&current_position.position).unwrap();
                current_position.position = match self.instructions[instruction_index] {
                    'L' => {
                        String::from(&crossing.left)
                    }
                    'R' => {
                        String::from(&crossing.right)
                    }
                    _ => {
                        panic!("Invalid instruction");
                    }
                };
                index += 1;
                instruction_index = (instruction_index + 1) % self.instructions.len();
                current_position.instruction_index = instruction_index;
            }

            let cycle_start = visited_positions.iter().position(|pos| pos == &current_position).unwrap().to_u32().unwrap();
            cycle_lenghts.push(index - cycle_start);
        }

        println!("Cycle lengths: {:?}", cycle_lenghts);

        let factors = cycle_lenghts.iter().map(|x| Factorization::run(*x).factors);
        let mut prime_factors_hist = factors.map(|x| {
            let mut hist = HashMap::new();
            for factor in x {
                let count = hist.entry(factor).or_insert(0u32);
                *count += 1;
            }
            hist
        }).collect::<Vec<HashMap<u32, u32>>>();

        println!("Prime factors hist: {:?}", prime_factors_hist);

        let condensed_prime = prime_factors_hist.iter_mut().reduce(|a, b| {
            b.keys().for_each(|k| {
                let val = a.get(k).unwrap_or(&&0);
                a.insert(*k, *val.max(b.get(k).unwrap()));
            });
            a
        }).expect("No prime factors found");

        println!("Condensed prime factors: {:?}", condensed_prime);

        let mut steps = BigUint::from(1u32);
        for (key, value) in condensed_prime.iter() {
            steps = steps.mul(BigUint::from(*key).pow(*value));
        }
        steps
    }

    fn get_all_starting_positions(&self) -> Vec<&String> {
        let mut positions = Vec::new();
        for (key, _) in self.map.iter() {
            if key.ends_with("A") {
                positions.push(key);
            }
        }
        positions
    }
}

fn main() {
    let scenario = Scenario::read_from_file();
    println!("Instructions: {:?}", scenario.instructions);
    let steps = scenario.follow_instructions_till_zzz();
    println!("Steps to reach ZZZ: {}", steps);
}