use std::{collections::HashMap, fs};

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part2(&input);
    println!("Part 2: {output}");
}

fn part2(data: &str) -> u32 {
    let instructions = data.lines().next().unwrap().chars();
    let mut elements: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in data.lines().skip(2) {
        let location = line.split(" = (").next().unwrap();
        let directions: (&str, &str) = line
            .split(" = (")
            .last()
            .unwrap()
            .split(")")
            .next()
            .unwrap()
            .split_once(", ")
            .unwrap();

        elements.insert(location, directions);
    }

    let mut curr: Vec<&str> = elements
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect();
    let end = 'Z';
    let mut count = 0;
    let mut instruct_it = instructions.cycle();

    loop {
        if curr.iter().all(|n| n.ends_with(end)) {
            return count;
        }

        let next_val = instruct_it.next();

        let new_nodes: Vec<&str> = curr
            .iter()
            .map(|node| {
                let directions = elements.get(node).unwrap();
                match next_val {
                    Some('L') => directions.0,
                    Some('R') => directions.1,
                    _ => unimplemented!(),
                }
            })
            .collect();

        curr = new_nodes;
        count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA_FILE: &str = "test_data_part2.txt";

    fn load_file() -> String {
        fs::read_to_string(TEST_DATA_FILE).unwrap()
    }

    #[test]
    fn part2_works() {
        let data = load_file();
        let output = part2(&data);

        assert_eq!(output, 6);
    }
}
