use std::{collections::HashSet, fs};

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part2(&input);
    println!("Part 2: {output}");
}

fn count_empty_rows(space: &[Vec<char>], point1: &Point, point2: &Point) -> u32 {
    let start_pt = point1.min(point2);
    let end_pt = point1.max(point2);
    let mut empty = 0;
    let mut count = start_pt.y;
    loop {
        if count >= end_pt.y {
            break;
        }

        let row = &space[count as usize];
        if row.iter().all(|&ch| ch == '.') {
            empty += 1;
            count += 1;
        }
        count += 1;
    }
    empty
}

fn count_empty_cols(space: &[Vec<char>], point1: &Point, point2: &Point) -> u32 {
    let start_pt = point1.x.min(point2.x);
    let end_pt = point1.x.max(point2.x);
    let max_row = space.len();

    let mut count = 0;

    for x in start_pt..end_pt {
        let mut is_empty = true;
        for y in 0..max_row {
            if space[y][x as usize] == '#' {
                is_empty = false;
                break;
            }
        }

        if is_empty {
            count += 1;
        }
    }

    count
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    y: i32,
    x: i32,
}

fn solve(space: &[Vec<char>], expansion: u32) -> u64 {
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
                        let empty_rows = count_empty_rows(&space, &point_1, &point_2);
                        let empty_cols = count_empty_cols(&space, &point_1, &point_2);

                        if col2 == '#' && point_1 != point_2 && !counted.contains(&point_2) {
                            let dist =
                                point_2.x.abs_diff(point_1.x) + point_2.y.abs_diff(point_1.y);
                            let empty = empty_cols + empty_rows;
                            let final_dist = (dist - empty) + (empty * expansion);
                            sum += final_dist as u64;
                        }
                    }
                }
                counted.insert(point_1);
            }
        }
    }
    sum
}

fn part2(data: &str) -> u64 {
    let space: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    dbg!(count_empty_rows(
        &space,
        &Point { y: 0, x: 0 },
        &Point {
            x: space[0].len() as i32,
            y: space.len() as i32
        }
    ));

    dbg!(count_empty_cols(
        &space,
        &Point { y: 0, x: 0 },
        &Point {
            x: space[0].len() as i32,
            y: space.len() as i32
        }
    ));

    solve(&space, 1000000)
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
        let space: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
        assert_eq!(solve(&space, 2), 374);
        assert_eq!(solve(&space, 10), 1030);
        assert_eq!(solve(&space, 100), 8410);
    }
}
