use std::fs;

const DATA_FILE: &str = "data.txt";

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

fn part2(data: &str) -> u32 {
    data.lines()
        .map(|card| {
            let nums = card.split(": ").last().unwrap();
            let mut nums_it = nums.split("| ").into_iter();
            let card_nums = nums_from_line(&mut nums_it);
            let winnning_nums = nums_from_line(&mut nums_it);
            println!("{card_nums:?} {winnning_nums:?}\n");

            let win_count = card_nums
                .iter()
                .filter(|num| winnning_nums.contains(num))
                .count() as u32;

            if win_count == 0 {
                0
            } else {
                2u32.pow(win_count - 1)
            }
        })
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
        let output = part2(&data);

        assert_eq!(output, 30);
    }
}
