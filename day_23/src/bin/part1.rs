use std::{
    collections::{BinaryHeap, HashSet},
    fs,
    ops::Add,
};

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    y: i32,
    x: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

const fn new_pt(x: i32, y: i32) -> Point {
    Point { y, x }
}

// up, down, left, right
const DIRS: [Point; 4] = [new_pt(0, -1), new_pt(0, 1), new_pt(-1, 0), new_pt(1, 0)];

fn longest_hike(grid: &[Vec<u8>], start: &Point, end: &Point) -> u32 {
    let mut visited: HashSet<&Point> = HashSet::new();
    let mut points: BinaryHeap<(u32, &Point)> = BinaryHeap::new();

    points.push((0, start));
    visited.insert(start);

    while !points.is_empty() {
        let (dist, current) = points.pop().unwrap();
        if current == end {
            break;
        }
    }

    todo!()
}

fn part1(data: &str) -> u32 {
    let grid: Vec<Vec<_>> = data.lines().map(|line| line.bytes().collect()).collect();
    let start = new_pt(1, 0);
    let max_y = grid.len() as i32;
    let max_x = grid[0].len() as i32;
    let end = new_pt(max_x - 2, max_y - 1);

    longest_hike(&grid, &start, &end)
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

        assert_eq!(output, 94);
    }
}
