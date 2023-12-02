use std::{collections::HashMap, fs};

const DATA_FILE: &str = "data.txt";
const TEST_DATA_FILE: &str = "test_data.txt";

const GREEN_MAX: u8 = 13;
const RED_MAX: u8 = 12;
const BLUE_MAX: u8 = 14;
fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

fn game_is_valid(game: &str) -> bool {
    let rounds: Vec<&str> = game.split("; ").collect();

    rounds
        .iter()
        .map(|round| {
            let cubes: Vec<&str> = round.split(", ").collect();
            let mut cube_amount: HashMap<&str, u8> = HashMap::new();
            for cube in cubes {
                let num = cube.split(' ').next().unwrap();
                let colour = cube.split(' ').last().unwrap();
                cube_amount.insert(colour, num.parse::<u8>().unwrap());
            }
            if cube_amount.get("red").unwrap_or(&0) > &RED_MAX
                || cube_amount.get("blue").unwrap_or(&0) > &BLUE_MAX
                || cube_amount.get("green").unwrap_or(&0) > &GREEN_MAX
            {
                false
            } else {
                true
            }
        })
        .all(|x| x)
}

fn part1(data: &str) -> u32 {
    data.lines()
        .enumerate()
        .filter_map(|(i, game)| {
            let start = game.find(": ").unwrap();
            println!("");
            if game_is_valid(&game[start + 2..]) {
                Some((i + 1) as u32)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_file() -> String {
        fs::read_to_string(TEST_DATA_FILE).unwrap()
    }

    #[test]
    fn part1_works() {
        let data = load_file();
        let output = part1(&data);

        assert_eq!(output, 8);
    }
}
