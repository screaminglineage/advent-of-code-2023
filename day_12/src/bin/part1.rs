use std::fs;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

fn check_arrangement(springs: &[char], nums: &[i32]) -> bool {
    let mut count = 0;
    let mut nums_it = nums.iter();
    for &ch in springs {
        if ch == '#' {
            count += 1;
        } else if count != 0 {
            match nums_it.next() {
                Some(&num) if num == count => (),
                _ => return false,
            }
            count = 0;
        }
    }

    true
}

const SPRINGS: [char; 2] = ['#', '.'];

fn count_springs(springs: Vec<char>, nums: &[i32]) -> i32 {
    let mut count = 0;
    for (i, &ch) in springs.iter().enumerate() {
        if ch == '?' {
            for spring_type in SPRINGS {
                let mut new = springs.clone();
                new[i] = spring_type;
                if check_arrangement(&new, nums) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part1(data: &str) -> i32 {
    for line in data.lines() {
        let mut line = line.split_whitespace();
        let springs = line.next().unwrap();
        let nums: Vec<i32> = line
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        dbg!(springs, nums);
    }
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

        assert_eq!(output, 21);
    }

    fn str_to_vec(data: &str) -> Vec<char> {
        data.chars().collect()
    }

    #[test]
    fn check_arrangement_works() {
        assert!(check_arrangement(&str_to_vec("#.#.###"), &[1, 1, 3]));
        assert!(check_arrangement(&str_to_vec(".#...#....###."), &[1, 1, 3]));
        assert!(check_arrangement(
            &str_to_vec(".#.###.#.######"),
            &[1, 3, 1, 6]
        ));
        assert!(check_arrangement(
            &str_to_vec("#....######..#####."),
            &[1, 6, 5]
        ));
        assert!(check_arrangement(&str_to_vec(".###.##....#"), &[3, 2, 1]));
    }
}
