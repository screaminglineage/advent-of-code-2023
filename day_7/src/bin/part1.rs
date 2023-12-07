use std::{collections::HashMap, fs};

use itertools::Itertools;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

#[derive(Debug, Copy, Clone)]
struct Hand<'a> {
    cards: &'a str,
    bid: u32,
}

fn count_cards(hand: &Hand) -> HashMap<char, u8> {
    let mut card_count: HashMap<char, u8> = HashMap::new();
    for card in hand.cards.chars() {
        card_count.entry(card).and_modify(|c| *c += 1).or_insert(1);
    }
    card_count
}

fn part1(data: &str) -> u32 {
    let hands: Vec<Hand> = data
        .lines()
        .map(|line| {
            let line: Vec<&str> = line.split_whitespace().collect();
            Hand {
                cards: line[0],
                bid: line[1].parse::<u32>().unwrap(),
            }
        })
        .collect();

    let five_of_kinds: Vec<&Hand> = hands
        .iter()
        .filter(|hand| {
            let card_count = count_cards(hand);
            card_count.values().any(|c| *c == 5)
        })
        .collect();

    let four_of_kinds: Vec<&Hand> = hands
        .iter()
        .filter(|hand| {
            let card_count = count_cards(hand);
            card_count.values().any(|c| *c == 4)
        })
        .collect();

    let full_houses: Vec<&Hand> = hands
        .iter()
        .filter(|hand| {
            let card_count = count_cards(hand);
            card_count.values().any(|c| *c == 3)
                && card_count.values().filter(|c| **c != 3).all(|c| *c == 2)
        })
        .collect();

    let three_of_kinds: Vec<&Hand> = hands
        .iter()
        .filter(|hand| {
            let card_count = count_cards(hand);
            card_count.values().any(|c| *c == 3)
                && card_count.values().filter(|c| **c != 3).all(|c| *c == 1)
        })
        .collect();

    let two_pairs: Vec<&Hand> = hands
        .iter()
        .filter(|hand| {
            let card_count = count_cards(hand);
            card_count.values().filter(|c| **c == 2).count() == 2
        })
        .collect();

    let one_pairs: Vec<&Hand> = hands
        .iter()
        .filter(|hand| {
            let card_count = count_cards(hand);
            card_count.values().filter(|c| **c == 2).count() == 1
                && card_count.values().filter(|c| **c == 1).count() == 3
        })
        .collect();

    let high_cards: Vec<&Hand> = hands
        .iter()
        .filter(|hand| {
            let card_count = count_cards(hand);
            card_count.values().all(|c| *c == 1)
        })
        .collect();

    dbg!(five_of_kinds);
    dbg!(four_of_kinds);
    dbg!(full_houses);
    dbg!(three_of_kinds);
    dbg!(two_pairs);
    dbg!(one_pairs);
    dbg!(high_cards);

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA_FILE: &str = "test_data.txt";

    fn load_file() -> String {
        fs::read_to_string(TEST_DATA_FILE).unwrap()
    }

    #[test]
    fn part1_works() {
        let data = load_file();
        let output = part1(&data);

        assert_eq!(output, 6440);
    }
}
