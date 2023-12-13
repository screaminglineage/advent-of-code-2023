use std::fs;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

fn detect_mirror(pattern: &[Vec<char>]) -> Option<u32> {
    let indices: Vec<_> = pattern
        .windows(2)
        .enumerate()
        .filter(|(_, pair)| pair[0].iter().eq(pair[1].iter()))
        .map(|(i, _)| i)
        .collect();

    for index in indices {
        let (p1, p2) = pattern.split_at(index);
        if p1.iter().zip(p2.iter()).all(|(x, y)| x == y) {
            return Some(index as u32);
        }
    }
    None
}

fn count_rows_above(pattern: &str) -> u32 {
    let pattern: Vec<&str> = pattern.lines().collect();
    let mut front = 0;
    let mut back = pattern.len() - 1;
    let median = back / 2;
    let mut count = 0;

    let mut i = 0;
    while front <= median && back > median {
        if pattern[front] == pattern[back] {
            count += 1;
            front += 1;
            back -= 1;
        } else if i == 0 && pattern[front] == pattern[back - 1] {
            back -= 1;
        } else if i == 0 && pattern[front + 1] == pattern[back] {
            front += 1;
        } else {
            count = 0;
            break;
        }
        i += 1;
    }

    if count == 0 {
        count
    } else {
        count + 1
    }
}

fn count_rows_above_2(pattern: &[Vec<char>]) -> u32 {
    let mut front = 0;
    let mut back = pattern.len() - 1;
    let median = back / 2;
    let mut count = 0;

    let mut i = 0;
    while front <= median && back > median {
        if pattern[front] == pattern[back] {
            count += 1;
            front += 1;
            back -= 1;
        } else if i == 0 && pattern[front] == pattern[back - 1] {
            back -= 1;
        } else if i == 0 && pattern[front + 1] == pattern[back] {
            front += 1;
        } else {
            count = 0;
            break;
        }
        i += 1;
    }

    if count == 0 {
        count
    } else {
        count + 1
    }
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

fn part1(data: &str) -> u32 {
    let patterns: Vec<&str> = data.split("\n\n").collect();
    for p in &patterns {
        let a: Vec<Vec<char>> = p.lines().map(|line| line.chars().collect()).collect();

        let n = detect_mirror(&a);
        let a = rotate_90(a);
        let r = detect_mirror(&a);
        // for i in &a {
        //     println!("{i:?}")
        // }

        println!("not rotated: {:?}, rotated: {:?}", n, r);
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

        assert_eq!(output, 405);
    }
}
