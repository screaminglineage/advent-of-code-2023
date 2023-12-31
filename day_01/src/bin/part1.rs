use std::fs;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

fn part1(data: &str) -> i32 {
    data.lines()
        .map(|line| {
            let first_digit = line.chars().find(|c| c.is_numeric()).unwrap();
            let last_digit = line.chars().rev().find(|c| c.is_numeric()).unwrap();
            format!("{first_digit}{last_digit}").parse::<i32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    const TEST_DATA_FILE: &str = "test_data.txt";
    use super::*;

    fn load_file() -> String {
        fs::read_to_string(TEST_DATA_FILE).unwrap()
    }

    #[test]
    fn part1_works() {
        let data = load_file();
        let first_half = part1(&data);

        assert_eq!(first_half, 142);
    }
}
