use std::{collections::HashMap, fs};

const DATA_FILE: &str = "data.txt";
const TEST_DATA_FILE: &str = "test_data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part2(&input);
    println!("Part 2: {output}");
}

fn max_cubes(game: &str) -> u32 {
    let rounds: Vec<&str> = game.split("; ").collect();
    let mut max_cubes: HashMap<&str, u32> = HashMap::new();
    rounds
        .iter()
        .flat_map(|round| {
            let cubes: Vec<&str> = round.split(", ").collect();
            cubes
        })
        .for_each(|cube| {
            let num = cube.split(' ').next().unwrap().parse::<u32>().unwrap();
            let colour = cube.split(' ').last().unwrap();
            max_cubes
                .entry(colour)
                .and_modify(|n| {
                    if *n < num {
                        *n = num
                    }
                })
                .or_insert(num);
        });

    max_cubes.into_values().product()
}

fn part2(data: &str) -> u32 {
    data.lines()
        .map(|game| {
            let start = game.find(": ").unwrap();
            println!();
            max_cubes(&game[start + 2..])
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
    fn part2_works() {
        let data = load_file();
        let output = part2(&data);

        assert_eq!(output, 2286);
    }
}
