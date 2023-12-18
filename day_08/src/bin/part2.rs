use std::{collections::HashMap, fs, iter::Cycle, str::Chars};

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part2(&input);
    println!("Part 2: {output}");
}

fn count_steps(
    node: &str,
    instruct_it: &mut Cycle<Chars<'_>>,
    elements: &HashMap<&str, (&str, &str)>,
) -> u64 {
    let mut count = 0;
    let mut curr = node;
    loop {
        if curr.ends_with('Z') {
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

fn hcf(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        hcf(b, a % b)
    }
}

fn lcm(nums: Vec<u64>) -> u64 {
    nums.iter()
        .skip(1)
        .fold(nums[0], |acc, &num| (acc * num) / hcf(acc, num))
}

fn part2(data: &str) -> u64 {
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

    let nodes: Vec<&str> = elements
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect();

    let all_counts: Vec<u64> = nodes
        .iter()
        .map(|node| {
            let mut instruct_it = instructions.clone().cycle();
            count_steps(node, &mut instruct_it, &elements)
        })
        .collect();

    lcm(all_counts)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA_FILE: &str = "test_data_part2.txt";

    fn load_file() -> String {
        fs::read_to_string(TEST_DATA_FILE).unwrap()
    }

    #[test]
    fn lcm_works() {
        assert_eq!(lcm(vec![6, 15]), 30);
    }

    #[test]
    fn hcf_works() {
        assert_eq!(hcf(6, 15), 3);
        assert_eq!(hcf(318, 265), 53);
    }

    #[test]
    fn part2_works() {
        let data = load_file();
        let output = part2(&data);

        assert_eq!(output, 6);
    }
}
