use regex::Regex;

use advent_of_code_2023::read_resource_lines;

fn f(x: &u32, t: &u32) -> u32 {
    (t - x) * x
}


fn get_possible_ways_to_win(t: &u32, d: &u32) -> u32 {
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
    let times = regex.split(&lines[0]).skip(1).map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let distances = regex.split(&lines[1]).skip(1).map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut total_product = 1;
    for (t, d) in times.iter().zip(distances.iter()) {
        let possible_ways_to_win = get_possible_ways_to_win(t, d);
        total_product *= possible_ways_to_win;
    }
    println!("Total product: {}", total_product);
}