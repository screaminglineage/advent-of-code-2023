use std::{collections::HashSet, fs, thread::current};

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

fn calc_edges(directions: &[(char, i32)]) -> (HashSet<(i32, i32)>, u32) {
    let mut total = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    // current is in the order (y, x)
    let mut current = (0, 0);

    for (dir, count) in directions {
        match dir {
            'U' => current.0 -= count,
            'D' => current.0 += count,
            'L' => current.1 -= count,
            'R' => current.1 += count,
            a => panic!("got {a}"),
        }
        visited.insert(current);
        total += *count as u32;
    }
    (visited, total)
}

fn part1(data: &str) -> u32 {
    let directions: Vec<(char, i32)> = data
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
                    .unwrap() as i32,
            )
        })
        .collect();

    let (visited, edges) = calc_edges(&directions);

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

        let output = calc_edges(
            &data
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
                            .unwrap() as i32,
                    )
                })
                .collect::<Vec<(char, i32)>>(),
        );

        assert_eq!(output.1, 38);
    }

    #[test]
    fn part1_works() {
        let data = load_file();
        let output = part1(&data);

        assert_eq!(output, 62);
    }
}
