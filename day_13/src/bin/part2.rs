use std::fs;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part2(&input);
    println!("Part 2: {output}");
}

fn get_off_by_one(pattern: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut indices = Vec::new();
    for (i, lines) in pattern.windows(2).enumerate() {
        let [x, y] = lines else { unreachable!() };
        let mut count = 0;
        let mut index = 0;
        for (j, (a, b)) in x.iter().zip(y.iter()).enumerate() {
            if a != b {
                count += 1;
                index = j;
            }
        }
        if count == 1 {
            indices.push((i, index));
        }
    }
    indices
}

fn get_similar(pattern: &[Vec<char>]) -> Vec<usize> {
    pattern
        .windows(2)
        .enumerate()
        .filter(|(_, pair)| pair[0].iter().eq(pair[1].iter()))
        .map(|(i, _)| i)
        .collect()
}

fn count_mirror_rows(pattern: &mut [Vec<char>]) -> Option<u32> {
    let indices = get_similar(pattern);
    let indices2 = get_off_by_one(pattern);

    // for x in pattern.iter() {
    //     println!("{x:?}")
    // }

    // dbg!(indices2);

    for (index, _) in indices2 {
        let (p1, p2) = pattern.split_at(index + 1);

        let mut count = 0;
        for (a, b) in p1.iter().rev().zip(p2.iter()) {
            let is_off_by_one = a.iter().zip(b.iter()).filter(|(x, y)| x != y).count() == 1;
            if is_off_by_one {
                count += 1;
            }
        }
        if count <= 1 {
            return Some(index as u32 + 1);
        }
    }

    for index in indices {
        let (p1, p2) = pattern.split_at(index + 1);

        let mut count = 0;
        for (a, b) in p1.iter().rev().zip(p2.iter()) {
            let is_off_by_one = a.iter().zip(b.iter()).filter(|(x, y)| x != y).count() == 1;
            if is_off_by_one {
                count += 1;
            }
        }
        if count <= 1 {
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

// Passes test cases but not actual input
fn part2(data: &str) -> u32 {
    let patterns: Vec<&str> = data.split("\n\n").collect();
    let mut sum = 0;

    for (i, p) in patterns.iter().enumerate() {
        println!("Pattern {}", i + 1);
        let mut a: Vec<Vec<char>> = p.lines().map(|line| line.chars().collect()).collect();

        let row = count_mirror_rows(&mut a);

        let mut a = rotate_90(a);
        let col = count_mirror_rows(&mut a);

        dbg!(row, col);

        sum += match (row, col) {
            (Some(x), _) => x * 100,
            (None, Some(x)) => x,
            (None, None) => unreachable!(),
        }
    }
    // let mut a: Vec<Vec<char>> = patterns[1]
    //     .lines()
    //     .map(|line| line.chars().collect())
    //     .collect();
    // dbg!(count_mirror_rows(&mut a));
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
