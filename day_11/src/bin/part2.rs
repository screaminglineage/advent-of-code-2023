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
    let mut expansions = 0;
    let mut count = start_pt.y;
    loop {
        if count >= end_pt.y {
            break;
        }

        let row = &space[count as usize];
        if row.iter().all(|&ch| ch == '.') {
            expansions += 1;
            count += 1;
        }
        count += 1;
    }
    expansions
}

// fn count_empty_rows(space: &[Vec<char>], point1: &Point, point2: &Point) -> u32 {
//     let start_pt = point1.x.min(point2.x);
//     let end_pt = point1.x.max(point2.x);
//
//     for
//
//     0
// }

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

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    y: i32,
    x: i32,
}

fn part2(data: &str) -> u64 {
    let space: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    // dbg!(count_empty_rows(
    //     &space,
    //     &Point { y: 0, x: 0 },
    //     &Point {
    //         x: space[0].len() as i32,
    //         y: space.len() as i32
    //     }
    // ));
    //
    let space = rotate_90(space);
    // dbg!(count_empty_rows(
    //     &space,
    //     &Point { y: 0, x: 0 },
    //     &Point {
    //         x: space[0].len() as i32,
    //         y: space.len() as i32
    //     }
    // ));

    let inc = 1000000;

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
                        let rotated = rotate_90(space.clone());
                        let empty_cols = count_empty_rows(&rotated, &point_1, &point_2);

                        if col2 == '#' && point_1 != point_2 && !counted.contains(&point_2) {
                            let dist =
                                point_2.x.abs_diff(point_1.x) + point_2.y.abs_diff(point_1.y);
                            let empty = empty_cols + empty_rows;
                            let final_dist = (dist - empty) + (empty * inc);
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
