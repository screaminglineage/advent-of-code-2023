use std::fs;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part2(&input);
    println!("Part 2: {output}");
}

fn line_to_nums(line: &str) -> u64 {
    line.split(':')
        .last()
        .unwrap()
        .chars()
        .filter(|s| s.is_numeric())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

fn part2(data: &str) -> u64 {
    let lines: Vec<&str> = data.lines().collect();
    let time = line_to_nums(lines[0]);
    let dist = line_to_nums(lines[1]);

    (1..time)
        .filter(|i| {
            let speed = i;
            let time_left = time - i;
            let my_dist = speed * time_left;
            my_dist > dist
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA_FILE: &str = "test_data.txt";

    fn load_file() -> String {
        fs::read_to_string(TEST_DATA_FILE).unwrap()
    }

    #[test]
    fn part2_works() {
        let data = load_file();
        let output = part2(&data);

        assert_eq!(output, 71503);
    }
}
