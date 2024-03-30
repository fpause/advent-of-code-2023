use advent_of_code_2023::read_resource_lines;

#[derive(Clone)]
struct MapEntry {
    destination_start: u64,
    source_start: u64,
    range_length: u64,
}

#[derive(Clone)]
struct Map {
    entries: Vec<MapEntry>,
}

struct GardeningScenario {
    initial_seeds: Vec<u64>,
    maps: Vec<Map>,
}

fn extract_initial_seeds(line: &str) -> Vec<u64> {
    line.split(" ").skip(1).map(|x| x.parse::<u64>().unwrap()).collect()
}

fn extract_maps(lines: &Vec<String>) -> Vec<Map> {
    let mut maps = Vec::new();
    let mut current_map = Map { entries: Vec::new() };
    for line in lines {
        if line.contains(":") {
            current_map = Map { entries: Vec::new() }
        } else if line.is_empty() {
            maps.push(current_map.clone());
            continue;
        } else {
            let split = line.split(" ").collect::<Vec<&str>>();
            let map_entry = MapEntry {
                destination_start: split[0].parse::<u64>().unwrap(),
                source_start: split[1].parse::<u64>().unwrap(),
                range_length: split[2].parse::<u64>().unwrap(),
            };
            current_map.entries.push(map_entry.clone());
        }
    }
    maps.push(current_map.clone());
    maps
}

fn read_input_data() -> GardeningScenario {
    let mut lines = read_resource_lines("5.txt");
    let initial_seeds = extract_initial_seeds(&lines.remove(0));
    lines.remove(0);
    let maps = extract_maps(&lines);
    let gardening_scenario = GardeningScenario {
        initial_seeds,
        maps,
    };
    println!("{:?}", gardening_scenario.maps.len());
    gardening_scenario
}

impl Map {
    fn map_value(&self, value: u64) -> u64 {
        let matching_entry = self.entries.iter().filter(|entry| value >= entry.source_start && value < entry.source_start + entry.range_length).collect::<Vec<&MapEntry>>();
        if matching_entry.len() > 1 {
            println!("Error - Multiple matching entries found for value: {}", value);
        }
        if matching_entry.len() > 0 {
            return matching_entry[0].destination_start + value - matching_entry[0].source_start;
        }
        value
    }
}

fn main() {
    let garden_scenario = read_input_data();
    let mut result = Vec::new();
    for seed in garden_scenario.initial_seeds {
        let mut value = seed;
        for map in &garden_scenario.maps {
            value = map.map_value(value);
        }
        result.push(value);
    }
    println!("{:?}", result);
    println!("{:?}", result.iter().min().unwrap());
}