use std::{collections::HashMap, fs};

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

fn part1(data: &str) -> u32 {
    let instructions = data.lines().next().unwrap().chars();
    let mut elements: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in data.lines().skip(2) {
        let location = line.split(" = (").next().unwrap();
        let directions: (&str, &str) = line
            .split(" = (")
            .last()
            .unwrap()
            .split(')')
            .next()
            .unwrap()
            .split_once(", ")
            .unwrap();

        elements.insert(location, directions);
    }

    let mut curr = "AAA";
    let end = "ZZZ";
    let mut count = 0;
    let mut instruct_it = instructions.cycle();

    loop {
        if curr == end {
            return count;
        }

        let directions = elements.get(&curr).unwrap();
        curr = match instruct_it.next() {
            Some('L') => directions.0,
            Some('R') => directions.1,
            _ => unimplemented!(),
        };
        count += 1;
    }
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

        assert_eq!(output, 2);
    }
}
