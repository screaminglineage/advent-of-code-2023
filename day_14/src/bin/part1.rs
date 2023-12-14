use std::fs;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

fn roll_rocks(mut platform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for (y, row) in platform.iter().enumerate() {
        for (x, &unit) in row.iter().enumerate() {
            if unit != '.' {
                continue;
            }
        }
    }
    todo!()
}

fn part1(data: &str) -> u32 {
    let platform: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let platform = roll_rocks(platform);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA_FILE: &str = "test_data.txt";

    fn load_file() -> String {
        fs::read_to_string(TEST_DATA_FILE).unwrap()
    }

    fn vecs_match<T: PartialEq>(a: &[Vec<T>], b: &[Vec<T>]) -> bool {
        assert!(!a.is_empty() && !b.is_empty());
        if a.len() != b.len() || a[0].len() != b[0].len() {
            return false;
        }
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }

    #[test]
    fn roll_rocks_works() {
        let data = load_file();
        let rolled_rocks = roll_rocks(&data);
        let answer = [
            vec!['O', 'O', 'O', 'O', '.', '#', '.', 'O', '.', '.'],
            vec!['O', 'O', '.', '.', '#', '.', '.', '.', '.', '#'],
            vec!['O', 'O', '.', '.', 'O', '#', '#', '.', '.', 'O'],
            vec!['O', '.', '.', '#', '.', 'O', 'O', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '#', '.', '#'],
            vec!['.', '.', 'O', '.', '.', '#', '.', 'O', '.', 'O'],
            vec!['.', '.', 'O', '.', '.', '.', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '.', '#', '#', '#', '.', '.'],
            vec!['#', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
        ];
        assert!(vecs_match(&answer, &rolled_rocks));
    }

    #[test]
    fn part1_works() {
        let data = load_file();
        let output = part1(&data);

        assert_eq!(output, 136);
    }
}
