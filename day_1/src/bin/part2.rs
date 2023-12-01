use std::fs;

const DATA_FILE: &str = "data.txt";
const TEST_DATA_FILE: &str = "test_data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part2(&input);
    println!("Part 2: {output}");
}

fn part2(data: &str) -> String {
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
    fn part2_works() {
        let data = load_file();
        let first_half = first_half(&data);

        assert_eq!(first_half, "");
    }
}
