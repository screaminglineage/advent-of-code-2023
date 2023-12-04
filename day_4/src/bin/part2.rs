use std::collections::{HashMap, VecDeque};
use std::fs;

const DATA_FILE: &str = "data.txt";
const MAX_CARD: u32 = 219;

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part2(&input);
    println!("Part 2: {output}");
}

fn nums_from_line<'a, I: Iterator<Item = &'a str>>(nums_it: &mut I) -> Vec<u32> {
    nums_it
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn get_card_num(card_line: &str) -> u32 {
    card_line
        .split(": ")
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap()
}

#[derive(Debug, Clone)]
struct Card {
    card_id: u32,
    card_nums: Vec<u32>,
    winnning_nums: Vec<u32>,
    count: u32,
}

fn parse_card(card_line: &str) -> Card {
    let card_id = get_card_num(card_line);
    let nums = card_line.split(": ").last().unwrap();
    let mut nums_it = nums.split("| ").into_iter();
    let card_nums = nums_from_line(&mut nums_it);
    let winnning_nums = nums_from_line(&mut nums_it);
    Card {
        card_id,
        card_nums,
        winnning_nums,
        count: 1,
    }
}

fn get_win_count(card: &Card) -> u32 {
    card.card_nums
        .iter()
        .filter(|num| card.winnning_nums.contains(num))
        .count() as u32
}

// damn this is so slow
fn part2(data: &str) -> u32 {
    let mut cards: HashMap<u32, Card> = HashMap::new();
    let mut cards_to_process: VecDeque<u32> = VecDeque::new();

    for card in data.lines() {
        let card = parse_card(card);
        let win_count = get_win_count(&card);
        cards.insert(card.card_id, card.clone());

        let cards_won = card.card_id + 1..card.card_id + win_count + 1;
        cards_to_process.extend(cards_won);
    }

    while !cards_to_process.is_empty() {
        let card_id = cards_to_process.pop_front().unwrap();
        let card = cards.get(&card_id).unwrap();

        let win_count = get_win_count(&card);
        cards.entry(card_id).and_modify(|card| card.count += 1);

        let cards_won = card_id + 1..card_id + win_count + 1;
        cards_to_process.extend(cards_won);
    }

    cards.values().map(|card| card.count).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA_FILE: &str = "test_data.txt";

    fn load_file() -> String {
        fs::read_to_string(TEST_DATA_FILE).unwrap()
    }

    #[test]
    fn part2_works() {
        let data = load_file();
        let output = part2(&data);

        assert_eq!(output, 30);
    }
}
