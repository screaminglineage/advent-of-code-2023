use std::fs;

const DATA_FILE: &str = "data.txt";

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
            cubes
                .iter()
                .map(|cube| {
                    let num = cube.split(' ').next().unwrap();
                    let num = num.parse::<u8>().unwrap();
                    let colour = cube.split(' ').last().unwrap();
                    match colour {
                        "red" if num <= RED_MAX => true,
                        "green" if num <= GREEN_MAX => true,
                        "blue" if num <= BLUE_MAX => true,
                        _ => false,
                    }
                })
                .all(|x| x)
        })
        .all(|x| x)
}

fn part1(data: &str) -> u32 {
    data.lines()
        .enumerate()
        .filter_map(|(i, game)| {
            let start = game.find(": ").unwrap();
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
    const TEST_DATA_FILE: &str = "test_data.txt";

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
