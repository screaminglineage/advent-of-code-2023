use std::{collections::HashSet, fs};

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part2(&input);
    println!("Part 2: {output}");
}

fn expand_rows(space: &mut Vec<Vec<char>>) -> u32 {
    let mut expansions = 0;
    let mut count = 0;
    loop {
        if count >= space.len() - 1 {
            break;
        }

        let row = &space[count];
        if row.iter().all(|&ch| ch == '.') {
            let new_row = vec!['.'; row.len()];
            if count < row.len() {
                space.insert(count + 1, new_row);
            } else {
                space.push(new_row);
            }
            expansions += 1;
            count += 1;
        }
        count += 1;
    }
    expansions
}

fn expand_columns(space: &mut Vec<Vec<char>>) -> u32 {
    let mut col = 0;
    let max_row = space.len();
    let mut expansions = 0;
    loop {
        if col >= space[0].len() - 1 {
            break;
        }

        let mut will_expand = true;
        for i in 0..max_row {
            if space[i][col] == '#' {
                will_expand = false;
                break;
            }
        }
        if will_expand {
            for i in 0..max_row {
                if i < max_row {
                    space[i].insert(col + 1, '.');
                } else {
                    space[i].push('.');
                }
            }
            expansions += 1;
            col += 1;
        }
        col += 1;
    }
    expansions
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    y: i32,
    x: i32,
}

fn part2(data: &str) -> u64 {
    let mut space: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    // let a = expand_rows(&mut space);
    // let b = expand_columns(&mut space);
    // dbg!(a, b);

    let mut counted = HashSet::new();
    let mut sum = 0;
    for (y1, row) in space.iter().enumerate() {
        for (x1, &col) in row.iter().enumerate() {
            if col == '#' {
                let point_1 = Point {
                    x: x1 as i32,
                    y: y1 as i32,
                };
                for (y2, row2) in space.iter().enumerate() {
                    for (x2, &col2) in row2.iter().enumerate() {
                        let point_2 = Point {
                            x: x2 as i32,
                            y: y2 as i32,
                        };
                        if col2 == '#' && point_1 != point_2 && !counted.contains(&point_2) {
                            let dist = (point_2.x.abs_diff(point_1.x) * 2)
                                .abs_diff(point_2.y.abs_diff(point_1.y) * 3);
                            sum += dist as u64;
                        }
                    }
                }
                counted.insert(point_1);
            }
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

        assert_eq!(output, 374);
    }
}
