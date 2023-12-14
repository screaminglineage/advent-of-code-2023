use std::fs;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

fn roll_rocks(platform: &mut Vec<Vec<char>>) {
    let max_row = platform.len();
    let max_col = platform[0].len();
    for y in 0..max_row {
        for x in 0..max_col {
            let current = platform[y][x];
            if current != '.' {
                continue;
            }
            let mut height = y;
            while height < max_row - 1 {
                if platform[height][x] == 'O' || platform[height][x] == '#' {
                    break;
                }
                height += 1;
            }
            if platform[height][x] == '#' {
                continue;
            } else if platform[height][x] == 'O' {
                platform[y][x] = platform[height][x];
                platform[height][x] = current;
            }
        }
    }
}

fn part1(data: &str) -> u32 {
    let mut platform: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    roll_rocks(&mut platform);

    let max = platform.len();
    platform
        .iter()
        .zip((1..max + 1).rev())
        .map(|(row, i)| row.iter().filter(|&&ch| ch == 'O').count() * i)
        .sum::<usize>() as u32
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
        let mut platform: Vec<Vec<char>> =
            data.lines().map(|line| line.chars().collect()).collect();
        roll_rocks(&mut platform);
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
        assert!(vecs_match(&answer, &platform));
    }

    #[test]
    fn part1_works() {
        let data = load_file();
        let output = part1(&data);

        assert_eq!(output, 136);
    }
}
