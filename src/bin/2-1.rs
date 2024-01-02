use std::str::FromStr;

use advent_of_code_2023::read_resource_lines;

#[derive(Debug)]
struct Game {
    id: i32,
    game_results: Vec<GameResult>,
}

#[derive(Debug)]
struct GameResult {
    red: i8,
    green: i8,
    blue: i8,
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

    let query_red = 12;
    let query_green = 13;
    let query_blue = 14;
    let cmp_game_result = GameResult {
        red: query_red,
        green: query_green,
        blue: query_blue,
    };
    let mut total = 0;
    for game_result in all_game_results {
        if is_possible(&game_result, &cmp_game_result) {
            total += game_result.id;
        }
    }
    println!("{}", total)
}

fn is_possible(game: &Game, cmp_game_result: &GameResult) -> bool {
    for game_result in &game.game_results {
        if game_result.red > cmp_game_result.red || game_result.green > cmp_game_result.green || game_result.blue > cmp_game_result.blue {
            return false;
        }
    }
    return true;
}

fn get_game_results(lines: Vec<String>) -> Vec<Game> {
    let mut all_game_results: Vec<Game> = Vec::new();

    for line in lines {
        let mut sp = line.split(":");
        let id = sp.next().unwrap().replace("Game", "").trim().parse().unwrap();
        let game_results_str = sp.next().unwrap();
        let mut game = Game {
            id,
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