use std::collections::HashMap;

use advent_of_code_2023::read_resource_lines;

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_rank = self.get_rank();
        let other_rank = other.get_rank();
        return if self_rank > other_rank {
            std::cmp::Ordering::Greater
        } else if self_rank < other_rank {
            std::cmp::Ordering::Less
        } else {
            for i in 0..self.cards.len() {
                let self_card = get_card_value(&self.cards[i]);
                let other_card = get_card_value(&other.cards[i]);
                if self_card > other_card {
                    return std::cmp::Ordering::Greater;
                } else if self_card < other_card {
                    return std::cmp::Ordering::Less;
                }
            }
            std::cmp::Ordering::Greater
        };
    }
}

impl Hand {
    fn hash_card_x_times(&self, times: i32) -> bool {
        let mut map = HashMap::new();
        for card in &self.cards {
            *map.entry(card).or_insert(0) += 1;
        }
        map.values().any(|&v| v == times)
    }
    fn is_five_of_a_kind(&self) -> bool {
        self.hash_card_x_times(5)
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.hash_card_x_times(4)
    }

    fn is_full_house(&self) -> bool {
        self.hash_card_x_times(3) && self.hash_card_x_times(2)
    }

    fn is_three_of_a_kind(&self) -> bool {
        self.hash_card_x_times(3)
    }

    fn is_two_pair(&self) -> bool {
        let mut map = HashMap::new();
        for card in &self.cards {
            *map.entry(card).or_insert(0) += 1;
        }
        map.values().filter(|&v| v == &2).collect::<Vec<&i32>>().len() == 2
    }

    fn is_one_pair(&self) -> bool {
        self.hash_card_x_times(2)
    }

    fn get_rank(&self) -> u8 {
        if self.is_five_of_a_kind() {
            return 6;
        }
        if self.is_four_of_a_kind() {
            return 5;
        }
        if self.is_full_house() {
            return 4;
        }
        if self.is_three_of_a_kind() {
            return 3;
        }
        if self.is_two_pair() {
            return 2;
        }
        if self.is_one_pair() {
            return 1;
        }
        return 0;
    }
}

fn main() {
    let lines = read_resource_lines("7.txt");
    let mut hands = parse_hands(&lines);
    hands.sort();
    println!("{:?}", hands);
    let mut index = 1;
    let mut total = 0;
    for hand in hands {
        total += hand.bid * index;
        index += 1;
    }
    println!("Total: {}", total);
}

fn get_card_value(c: &char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap(),
    }
}

fn parse_hands(lines: &Vec<String>) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in lines {
        let split = line.split(" ").collect::<Vec<&str>>();
        let mut cards = split[0].chars();
        let bid = split[1].parse::<u32>().unwrap();
        let cards = cards.collect();
        let hand = Hand { cards, bid };
        hands.push(hand);
    }
    hands
}