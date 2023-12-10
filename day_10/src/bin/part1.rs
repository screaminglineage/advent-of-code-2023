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

fn is_connected(grid: &[Vec<char>], curr: &Point, next: &Point) -> bool {
    let curr_char = grid[curr.y][curr.x];
    let next_char = grid[next.y][next.x];

    if curr_char == '.' || next_char == '.' {
        return false;
    }

    if curr_char == 'S' || next_char == 'S' {
        return true;
    }

    // towards east
    if curr.x < next.y && curr.y == next.y {
        return matches!((curr_char, next_char), ('-', '-') | ('-', 'J') | ('-', '7'));

    // towards west
    } else if curr.x > next.y && curr.y == next.y {
        return matches!((curr_char, next_char), ('-', '-') | ('-', 'F') | ('-', 'L'));

    // towards north
    } else if curr.x == next.y && curr.y > next.y {
        return matches!((curr_char, next_char), ('|', '|') | ('|', 'F') | ('|', '7'));

    // towards south
    } else if curr.x == next.y && curr.y < next.y {
        return matches!((curr_char, next_char), ('|', '|') | ('|', 'J') | ('|', 'L'));
    }

    false
}

fn traverse_grid(grid: &[Vec<char>], start: &Point) -> Vec<Vec<u32>> {
    let dists: Vec<Vec<u32>> = Vec::new();
    let mut points: VecDeque<(Point, u32)> = VecDeque::new();
    let mut visited: HashSet<Point> = HashSet::new();

    let point = start;
    let neighbours = validate_points(grid, &point, get_neighbours(grid, &point));
    neighbours.iter().for_each(|point| {
        visited.insert(*point);
    });

    points.extend(neighbours.into_iter().map(|p| (p, 1)));

    while !points.is_empty() {
        let (point, dist) = points.pop_front().unwrap();
        if visited.contains(&point) {
            continue;
        }
        let dist = dist + 1;

        let neighbours = validate_points(grid, &point, get_neighbours(grid, &point));

        neighbours.iter().for_each(|point| {
            visited.insert(*point);
        });
        points.extend(neighbours.into_iter().map(|p| (p, dist)));
        dbg!(&points);
    }

    dists
}

fn find_start(grid: &[Vec<char>]) -> Point {
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if grid[i][j] == 'S' {
                return Point { x: i, y: j };
            }
        }
    }
    panic!("Should always find the S");
}

fn part1(data: &str) -> u32 {
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let start = find_start(&grid);
    let dists = traverse_grid(&grid, &start);
    dbg!(&dists);
    // *dists.iter().flatten().max().unwrap()
    todo!()
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
    fn part1_works() {
        let data1 = load_file1();
        let data2 = load_file2();

        let output1 = part1(&data1);
        let output2 = part1(&data2);

        assert_eq!(output1, 4);
        assert_eq!(output2, 8);
    }
}
