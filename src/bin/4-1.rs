use advent_of_code_2023::read_resource_lines;

fn main() {
    let lines = read_resource_lines("4.txt");

    let mut total = 0;
    for line in lines {
        total += get_scratch_card_score(line);
    }

    println!("{}", total)
}

fn get_scratch_card_score(line: String) -> i32 {
    let (game_numbers, my_numbers) = get_lists(line);
    let mut score = 0;
    for my_number in my_numbers {
        if game_numbers.contains(&my_number) {
            if score == 0 {
                score = 1;
            } else {
                score <<= 1;
            }
        }
    }
    score
}

fn get_lists(line: String) -> (Vec<i32>, Vec<i32>) {
    let mut split = line.split("|");
    let game_numbers = split.next().unwrap();
    let mut game_numbers_split = game_numbers.split(":");
    game_numbers_split.next();
    return (get_numbers_list(game_numbers_split.next().unwrap()), get_numbers_list(split.next().unwrap()));
}

fn get_numbers_list(numbers: &str) -> Vec<i32> {
    numbers.trim().split_whitespace().map(|e| e.trim().parse::<i32>().unwrap()).collect()
}