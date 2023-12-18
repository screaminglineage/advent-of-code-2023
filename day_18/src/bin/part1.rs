use std::{collections::HashSet, fs};

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

fn calc_edges(directions: &[(char, u32)]) -> u32 {
    let mut total = 0;
    let visited: HashSet<(i32, i32)> = HashSet::new();
    let mut current = (0, 0);
    for (dir, count) in directions {
        match dir {
            'U' => (),
            'D' => (),
            'L' => (),
            'R' => (),
            _ => unreachable!(),
        }
    }
    total
}

fn part1(data: &str) -> u32 {
    let directions: Vec<(char, u32)> = data
        .lines()
        .map(|line| {
            let mut l = line.split_whitespace();
            (
                l.next().unwrap().chars().next().unwrap(),
                l.next()
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap()
                    .to_digit(10)
                    .unwrap(),
            )
        })
        .collect();

    dbg!(calc_edges(&directions));

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
    fn outer_edge() {
        let data = load_file();
        let output = calc_edges(&[('A', 32)]);

        assert_eq!(output, 38);
    }

    #[test]
    fn part1_works() {
        let data = load_file();
        let output = part1(&data);

        assert_eq!(output, 62);
    }
}
