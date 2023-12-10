use std::{
    collections::{HashSet, VecDeque},
    fs, usize,
};

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn get_neighbours(grid: &[Vec<char>], point: &Point) -> Vec<Point> {
    let mut neighbours = Vec::new();
    for i in -1i32..2i32 {
        for j in -1i32..2i32 {
            let y = point.y as i32 + i;
            let x = point.x as i32 + j;

            // skipping diagonals and the original point
            if i.abs() == j.abs() || (i == 0 && j == 0) {
                continue;
            }

            if y < 0 || x < 0 {
                continue;
            }

            if y >= grid.len() as i32 || x >= grid.first().unwrap().len() as i32 {
                continue;
            }
            neighbours.push(Point {
                x: x as usize,
                y: y as usize,
            });
        }
    }
    neighbours
}

fn __is_connected(grid: &[Vec<char>], curr: &Point, next: &Point) -> bool {
    let curr_char = grid[curr.y][curr.x];
    let next_char = grid[next.y][next.x];

    if curr_char == '.' || next_char == '.' {
        return false;
    }

    if curr_char == 'S' || next_char == 'S' {
        return true;
    }

    // towards east
    if curr.x < next.x && curr.y == next.y {
        return matches!(
            (curr_char, next_char),
            ('-', '-') | ('-', 'J') | ('-', '7') | ('L', 'J') | ('F', '7') | ('F', 'J')
        );

    // towards west
    } else if curr.x > next.x && curr.y == next.y {
        return matches!(
            (curr_char, next_char),
            ('-', '-') | ('-', 'F') | ('-', 'L') | ('7', 'L') | ('F', 'J') | ('F', '7')
        );

    // towards north
    } else if curr.x == next.x && curr.y > next.y {
        return matches!(
            (curr_char, next_char),
            ('|', '|') | ('|', 'F') | ('|', '7') | ('J', 'F') | ('J', '7') | ('J', '|')
        );

    // towards south
    } else if curr.x == next.x && curr.y < next.y {
        return matches!((curr_char, next_char), ('|', '|') | ('|', 'J') | ('|', 'L'));
    }

    false
}

fn is_connected(grid: &[Vec<char>], curr: &Point, next: &Point) -> bool {
    __is_connected(grid, curr, next) || __is_connected(grid, next, curr)
}

fn traverse_grid(grid: &[Vec<char>], start: &Point) -> u32 {
    let mut points: VecDeque<(Point, u32)> = VecDeque::new();
    let mut visited: HashSet<Point> = HashSet::new();

    dbg!(start);
    visited.insert(*start);
    points.push_back((*start, 0));

    let mut max_dist = 0;
    while !points.is_empty() {
        let (point, dist) = points.pop_front().unwrap();
        // println!("{:?} {}", point, grid[point.y][point.x]);
        visited.insert(point);

        for neighbour in get_neighbours(grid, &point) {
            if is_connected(grid, &point, &neighbour) && !visited.contains(&neighbour) {
                points.push_back((neighbour, dist + 1));
            }
        }
        if dist > max_dist {
            println!("{:?} {}", point, grid[point.y][point.x]);
            max_dist = dist;
        }
    }

    max_dist
}

fn find_start(grid: &[Vec<char>]) -> Point {
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if grid[i][j] == 'S' {
                return Point { x: j, y: i };
            }
        }
    }
    panic!("Should always find the S");
}

fn part1(data: &str) -> u32 {
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let start = find_start(&grid);
    traverse_grid(&grid, &start)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA_FILE1: &str = "test_data1.txt";
    const TEST_DATA_FILE2: &str = "test_data2.txt";

    fn load_file1() -> String {
        fs::read_to_string(TEST_DATA_FILE1).unwrap()
    }

    fn load_file2() -> String {
        fs::read_to_string(TEST_DATA_FILE2).unwrap()
    }

    #[test]
    fn connected_works1() {
        let data = load_file1();
        let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
        assert!(is_connected(
            &grid,
            &Point { y: 1, x: 1 },
            &Point { y: 2, x: 1 }
        ));

        assert!(is_connected(
            &grid,
            &Point { y: 1, x: 2 },
            &Point { y: 1, x: 3 }
        ));

        assert!(is_connected(
            &grid,
            &Point { y: 3, x: 1 },
            &Point { y: 3, x: 2 }
        ));
    }

    #[test]
    fn connected_works2() {
        let data = load_file2();
        let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
        assert!(is_connected(
            &grid,
            &Point { y: 3, x: 1 },
            &Point { y: 3, x: 2 }
        ));
    }

    #[test]
    fn part1_works() {
        let data1 = load_file1();
        let data2 = load_file2();

        // let output1 = part1(&data1);
        let output2 = part1(&data2);

        // assert_eq!(output1, 4);
        assert_eq!(output2, 8);
    }
}
