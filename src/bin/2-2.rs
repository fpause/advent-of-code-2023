use std::cmp::max;
use std::str::FromStr;

use advent_of_code_2023::read_resource_lines;

#[derive(Debug)]
struct Game {
    game_results: Vec<GameResult>,
}

#[derive(Debug)]
struct GameResult {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(input: &str) -> Result<Color, Self::Err> {
        match input {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

fn main() {
    let lines = read_resource_lines("2.txt");

    let all_game_results = get_game_results(lines);

    let mut total = 0;
    for game_result in all_game_results {
        let fewest_cubes_possible = get_fewest_cubes_possible(&game_result);
        total += get_power(&fewest_cubes_possible);
    }
    println!("{}", total)
}

fn get_fewest_cubes_possible(game: &Game) -> GameResult {
    let mut fewest_result = GameResult {
        red: 0,
        green: 0,
        blue: 0,
    };
    for game_result in &game.game_results {
        fewest_result.red = max(fewest_result.red, game_result.red);
        fewest_result.green = max(fewest_result.green, game_result.green);
        fewest_result.blue = max(fewest_result.blue, game_result.blue);
    }
    return fewest_result;
}

fn get_power(game_result: &GameResult) -> i32 {
    return game_result.red * game_result.green * game_result.blue;
}

fn get_game_results(lines: Vec<String>) -> Vec<Game> {
    let mut all_game_results: Vec<Game> = Vec::new();

    for line in lines {
        let mut sp = line.split(":");
        sp.next();
        let game_results_str = sp.next().unwrap();
        let mut game = Game {
            game_results: Vec::new(),
        };
        for game_result_split in game_results_str.split(";") {
            let mut game_result = GameResult { red: 0, green: 0, blue: 0 };
            for game_result_color in game_result_split.split(",") {
                let mut game_result_color_split = game_result_color.trim().split_whitespace();
                let amount = game_result_color_split.next().unwrap().parse().unwrap();
                let color = Color::from_str(game_result_color_split.next().unwrap()).unwrap();
                match color {
                    Color::Red => {
                        game_result.red = amount
                    }
                    Color::Green => {
                        game_result.green = amount
                    }
                    Color::Blue => {
                        game_result.blue = amount
                    }
                }
            }
            game.game_results.push(game_result);
        }
        all_game_results.push(game);
    }
    all_game_results
}