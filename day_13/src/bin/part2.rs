use std::fs;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part2(&input);
    println!("Part 2: {output}");
}

fn count_mirror_rows(pattern: &[Vec<char>]) -> Option<u32> {
    let indices: Vec<_> = pattern
        .windows(2)
        .enumerate()
        .filter(|(_, pair)| pair[0].iter().eq(pair[1].iter()))
        .map(|(i, _)| i)
        .collect();

    for index in indices {
        let (p1, p2) = pattern.split_at(index + 1);
        if p1.iter().rev().zip(p2.iter()).all(|(x, y)| x == y) {
            return Some(index as u32 + 1);
        }
    }
    None
}

fn rotate_90<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
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
        .rev()
        .collect()
}

fn part2(data: &str) -> u32 {
    let patterns: Vec<&str> = data.split("\n\n").collect();
    let mut sum = 0;
    for p in &patterns {
        let a: Vec<Vec<char>> = p.lines().map(|line| line.chars().collect()).collect();

        let row = count_mirror_rows(&a);
        let a = rotate_90(a);
        let col = count_mirror_rows(&a);
        sum += match (row, col) {
            (Some(x), None) => x * 100,
            (None, Some(x)) => x,
            _ => unreachable!("Input doesnt seem to have both horizontal and vertical mirroring"),
        }
    }
    sum
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

        assert_eq!(output, 400);
    }
}
