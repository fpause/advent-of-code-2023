use regex::Regex;

use advent_of_code_2023::read_resource_lines;

fn f(x: &u64, t: &u64) -> u64 {
    (t - x) * x
}


fn get_possible_ways_to_win(t: &u64, d: &u64) -> u64 {
    let mut possible_ways = 0;
    for x in 1..=*t {
        if &f(&x, t) > d {
            possible_ways += 1;
        }
    }
    possible_ways
}

fn main() {
    let lines = read_resource_lines("6.txt");
    let regex = Regex::new(r"\s+").unwrap();
    let time = regex.split(&lines[0]).skip(1).collect::<Vec<&str>>().join("").parse::<u64>().unwrap();
    let distance = regex.split(&lines[1]).skip(1).collect::<Vec<&str>>().join("").parse::<u64>().unwrap();
    println!("Time: {}, Distance: {}", time, distance);

    let possible_ways_to_win = get_possible_ways_to_win(&time, &distance);
    println!("Ways to win: {}", possible_ways_to_win);
}