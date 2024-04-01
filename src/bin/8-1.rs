use std::collections::HashMap;

use advent_of_code_2023::read_resource_lines;

struct Crossing {
    left: String,
    right: String,
}

struct Scenario {
    instructions: Vec<char>,
    map: HashMap<String, Crossing>,
}

impl Scenario {
    fn read_from_file() -> Scenario {
        let lines = read_resource_lines("8.txt");
        let instructions = lines[0].chars().collect();
        let mut map = HashMap::new();
        let mut index = 0;
        for line in lines[2..].iter() {
            let condensed = line.replace(" ", "").replace("(", "").replace(")", ",").replace("=", ",");
            let split = condensed.split(",").collect::<Vec<&str>>();
            let key = String::from(split[0]);
            let left = String::from(split[1]);
            let right = String::from(split[2]);
            map.insert(key, Crossing { left, right });
            index += 1;
        }
        Scenario {
            instructions,
            map,
        }
    }

    fn follow_instructions_till_zzz(&self) -> i32 {
        let mut steps = 0;
        let mut current_position = "AAA";
        let mut instruction_index = 0;
        while (current_position != "ZZZ") {
            let crossing = self.map.get(current_position).expect(format!("No map entry for key {}", current_position).as_str());
            let next_position = match self.instructions[instruction_index] {
                'L' => {
                    &crossing.left
                }
                'R' => {
                    &crossing.right
                }
                _ => {
                    panic!("Invalid instruction");
                }
            };
            current_position = next_position;
            instruction_index = (instruction_index + 1) % self.instructions.len();
            steps += 1;
        }
        steps
    }
}


fn main() {
    let scenario = Scenario::read_from_file();
    let steps = scenario.follow_instructions_till_zzz();
    println!("Steps to reach ZZZ: {}", steps);
}