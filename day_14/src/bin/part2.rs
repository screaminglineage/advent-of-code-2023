use std::fs;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part2(&input);
    println!("Part 2: {output}");
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

fn rotate_90_right<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .rev()
                .collect::<Vec<T>>()
        })
        .collect()
}

// Should work but way too slow due to brute force approach
fn part2(data: &str) -> u32 {
    let mut platform: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let cycles = 1000000000;
    // let cycles = 1000;
    for _ in 0..cycles {
        // println!("Cycle {}", i + 1);
        for _ in 0..4 {
            roll_rocks(&mut platform);

            // println!("After Rolling");
            // for a in &platform {
            //     println!("{a:?}");
            // }

            platform = rotate_90_right(platform);
            // println!("After Rotating");
            // for a in &platform {
            //     println!("{a:?}");
            // }
        }
    }
    for a in &platform {
        println!("{a:?}");
    }

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
    fn rotate_90_right_works() {
        let vec1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let vec2 = rotate_90_right(vec1);
        dbg!(&vec2);
        assert!(vecs_match(
            &vec2,
            &[vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3],]
        ));
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
    fn part2_works() {
        let data = load_file();
        let output = part2(&data);

        assert_eq!(output, 64);
    }
}
