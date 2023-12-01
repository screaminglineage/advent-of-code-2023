use std::fs;

const DATA_FILE: &str = "data.txt";
const TEST_DATA_FILE: &str = "test_data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

fn part1(data: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_file() -> String {
        use std::fs;
        fs::read_to_string(TEST_DATA_FILE).unwrap()
    }

    #[test]
    fn part1_works() {
        let data = load_file();
        let first_half = first_half(&data);

        assert_eq!(first_half, "");
    }
}
