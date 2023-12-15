use std::fs;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

fn calculate_hash(seq: &str) -> u32 {
    let mut curr = 0;
    for ch in seq.chars() {
        curr += ch as u32;
        curr *= 17;
        curr %= 256;
    }
    curr
}

fn part1(data: &str) -> u32 {
    let data: Vec<&str> = data.trim().split(',').collect();
    data.into_iter().map(calculate_hash).sum()
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

        assert_eq!(output, 1320);
    }
}
