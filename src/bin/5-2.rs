use advent_of_code_2023::read_resource_lines;

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

#[derive(Debug)]
struct RangeWithLocation {
    range: Range,
    location: i64,
}

#[derive(Clone)]
struct MapEntry {
    destination_start: i64,
    source_start: i64,
    range_length: i64,
}

#[derive(Clone)]
struct Map {
    entries: Vec<MapEntry>,
}

struct InitialSeedRange {
    start: i64,
    length: i64,
}

struct GardeningScenario {
    initial_seeds: Vec<InitialSeedRange>,
    maps: Vec<Map>,
}

fn extract_initial_seeds(line: &str) -> Vec<InitialSeedRange> {
    let input = line.split(" ").skip(1);
    let seeds = input.map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    seeds.chunks(2).map(|x| InitialSeedRange { start: x[0], length: x[1] }).collect::<Vec<InitialSeedRange>>()
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
                destination_start: split[0].parse::<i64>().unwrap(),
                source_start: split[1].parse::<i64>().unwrap(),
                range_length: split[2].parse::<i64>().unwrap(),
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
    gardening_scenario
}

impl GardeningScenario {
    fn test(&self, start: i64, end: i64, mut ranges: Vec<RangeWithLocation>) -> Vec<RangeWithLocation> {
        if self.process(end) - self.process(start) == end - start {
            ranges.push(RangeWithLocation { range: Range { start, end }, location: self.process(start) });
            println!("Adding range: {:?}", ranges.last().unwrap());
            return ranges;
        }
        let start_location = self.process(start);
        let mut binary_end = (end - start) / 2;
        while self.process(start + binary_end) - start_location != binary_end {
            binary_end /= 2;
        }
        while self.process(start + binary_end) - start_location == binary_end {
            binary_end += 1;
        }
        ranges.push(RangeWithLocation { range: Range { start, end: start + binary_end - 1 }, location: start_location });
        println!("Adding range: {:?}", ranges.last().unwrap());
        self.test(start + binary_end, end, ranges)
    }

    fn process(&self, value: i64) -> i64 {
        let mut tmp_value = value;
        for map in &self.maps {
            tmp_value = map.map_value(tmp_value);
        }
        tmp_value
    }
}

impl Map {
    fn map_value(&self, value: i64) -> i64 {
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

    let mut total_min_location = Vec::new();
    for seed in &garden_scenario.initial_seeds {
        let ranges = garden_scenario.test(seed.start, seed.start + seed.length, Vec::new());
        total_min_location.push(ranges.iter().map(|x| x.location).min().unwrap());
    }
    println!("Total min location: {:?}", total_min_location.iter().min());
}