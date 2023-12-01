use std::fs;

const DATA_FILE: &str = "data.txt";
const TEST_DATA_FILE: &str = "test_data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part2(&input);
    println!("Part 2: {output}");
}

fn part1(data: &str) -> i32 {
    let mut result = 0;
    for line in data.lines() {
        let first_digit = line.chars().find(|c| c.is_numeric()).unwrap();
        let last_digit = line.chars().rev().find(|c| c.is_numeric()).unwrap();
        let num = format!("{first_digit}{last_digit}").parse::<i32>().unwrap();
        result += num;
    }
    result
}

fn part2(data: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_file() -> String {
        use std::fs;
        fs::read_to_string(TEST_DATA_FILE).unwrap()
    }

    #[test]
    fn part2_works() {
        let data = load_file();
        let first_half = part2(&data);

        assert_eq!(first_half, 281);
    }
}
