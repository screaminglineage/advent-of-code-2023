use std::{fs, iter::zip};

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

fn line_to_nums(line: &str) -> Vec<u32> {
    line.split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}

fn part1(data: &str) -> u32 {
    let lines: Vec<&str> = data.lines().collect();
    let times = line_to_nums(lines[0]);
    let distances = line_to_nums(lines[1]);

    let mut total: u32 = 1;
    for (time, dist) in zip(times, distances) {
        let mut winning_dists = Vec::new();
        for i in 1..time {
            let speed = i;
            let time_left = time - i;
            let my_dist = speed * time_left;
            if my_dist > dist {
                winning_dists.push(my_dist);
            }
        }
        total *= winning_dists.len() as u32
    }
    total
}

// alternate version using functional style
#[allow(unused)]
fn part1_alternate(data: &str) -> u32 {
    let lines: Vec<&str> = data.lines().collect();
    let times = line_to_nums(lines[0]);
    let distances = line_to_nums(lines[1]);

    zip(times, distances)
        .map(|(time, dist)| {
            (1..time)
                .filter(|i| {
                    let speed = i;
                    let time_left = time - i;
                    let my_dist = speed * time_left;
                    my_dist > dist
                })
                .collect::<Vec<u32>>()
                .len() as u32
        })
        .product::<u32>()
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

        assert_eq!(output, 288);
    }
}
