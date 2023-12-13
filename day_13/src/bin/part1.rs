use std::fs;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

fn detect_mirror(pattern: &str) -> u32 {
    let pattern: Vec<&str> = pattern.lines().collect();
    let indices: Vec<_> = pattern
        .windows(2)
        .enumerate()
        .filter(|(_, pair)| pair[0].chars().eq(pair[1].chars()))
        .map(|(i, _)| i)
        .collect();
    dbg!(pattern.len());
    0
}

fn count_rows_above(pattern: &str) -> u32 {
    let pattern: Vec<&str> = pattern.lines().collect();
    let mut front = 0;
    let mut back = pattern.len() - 1;
    let median = dbg!(back / 2);
    let mut count = 0;
    while front <= median && back > median {
        if pattern[front] == pattern[back] {
            count += 1;
            front += 1;
            back -= 1;
        } else if pattern[front] == pattern[back - 1] {
            back -= 1;
        } else if pattern[front + 1] == pattern[back] {
            front += 1;
        } else {
            break;
        }
    }

    if count == 0 {
        count
    } else {
        count + 1
    }
}

fn part1(data: &str) -> u32 {
    let patterns: Vec<&str> = data.split("\n\n").collect();
    let a = count_rows_above(patterns[0]);
    dbg!(a);
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
    fn part1_works() {
        let data = load_file();
        let output = part1(&data);

        assert_eq!(output, 405);
    }
}
