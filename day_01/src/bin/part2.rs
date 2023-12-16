use std::fs;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part2(&input);
    println!("Part 2: {output}");
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part2(data: &str) -> u32 {
    let mut result = 0;
    for line in data.lines() {
        let mut digits_in_line: Vec<u32> = Vec::new();
        let mut buf = String::new();
        for ch in line.chars() {
            if ch.is_numeric() {
                digits_in_line.push(ch.to_digit(10).unwrap());
                continue;
            }

            buf.push(ch);
            for (i, digit) in DIGITS.iter().enumerate() {
                if buf.contains(digit) {
                    digits_in_line.push((i + 1) as u32);
                    buf.drain(..buf.len() - 1);
                }
            }
        }
        result += digits_in_line[0] * 10 + digits_in_line.last().unwrap();
    }
    result
}

#[cfg(test)]
mod tests {
    const TEST_DATA_FILE: &str = "test_data.txt";
    use super::*;

    fn load_file() -> String {
        fs::read_to_string(TEST_DATA_FILE).unwrap()
    }

    #[test]
    fn part2_works() {
        let data = load_file();
        let first_half = part2(&data);

        assert_eq!(first_half, 281);
    }
}
