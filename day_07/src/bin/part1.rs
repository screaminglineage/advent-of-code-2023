use std::{collections::HashMap, fs};

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

fn get_cards(hand: &str) -> Vec<Card> {
    hand.chars()
        .map(|card| match card {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            a => panic!("invalid card {a}"),
        })
        .collect()
}
fn count_cards(hand: &[Card]) -> HashMap<Card, u8> {
    let mut card_count: HashMap<Card, u8> = HashMap::new();
    for card in hand {
        card_count.entry(*card).and_modify(|c| *c += 1).or_insert(1);
    }
    card_count
}

fn part1(data: &str) -> u32 {
    let hands: Vec<Hand> = data
        .lines()
        .map(|line| {
            let line: Vec<&str> = line.split_whitespace().collect();
            Hand {
                cards: get_cards(line[0].trim()),
                bid: line[1].parse::<u32>().unwrap(),
            }
        })
        .collect();

    let mut five_of_kinds = Vec::new();
    let mut four_of_kinds = Vec::new();
    let mut three_of_kinds = Vec::new();
    let mut full_houses = Vec::new();
    let mut two_pairs = Vec::new();
    let mut one_pairs = Vec::new();
    let mut high_cards = Vec::new();

    for hand in hands.iter() {
        let card_count = count_cards(&hand.cards);

        if card_count.values().any(|&x| x == 5) {
            five_of_kinds.push(hand);
        } else if card_count.values().any(|&x| x == 4) {
            four_of_kinds.push(hand);
        } else if card_count.values().any(|&x| x == 3) && card_count.values().all(|&x| x != 2) {
            three_of_kinds.push(hand);
        } else if card_count.values().any(|&x| x == 3) && card_count.values().any(|&x| x == 2) {
            full_houses.push(hand);
        } else if card_count.values().filter(|c| **c == 2).count() == 2 {
            two_pairs.push(hand);
        } else if card_count.values().filter(|c| **c == 2).count() == 1 {
            one_pairs.push(hand);
        } else {
            high_cards.push(hand);
        }
    }

    five_of_kinds.sort();
    four_of_kinds.sort();
    full_houses.sort();
    three_of_kinds.sort();
    two_pairs.sort();
    one_pairs.sort();
    high_cards.sort();

    high_cards
        .iter()
        .chain(one_pairs.iter())
        .chain(two_pairs.iter())
        .chain(three_of_kinds.iter())
        .chain(full_houses.iter())
        .chain(four_of_kinds.iter())
        .chain(five_of_kinds.iter())
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum()
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
