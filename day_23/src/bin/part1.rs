use std::{
    collections::{BinaryHeap, HashSet, VecDeque},
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
    let mut visited: HashSet<Point> = HashSet::new();
    let mut points: VecDeque<(u32, Point, HashSet<Point>)> = VecDeque::new();
    let mut dists = Vec::new();

    visited.insert(*start);
    points.push_back((0, *start, visited.clone()));

    let max_y = grid.len() as i32;
    let max_x = grid[0].len() as i32;

    while !points.is_empty() {
        let (dist, current, mut visited) = points.pop_front().unwrap();
        if current == *end {
            dists.push(dist);
            continue;
        }

        for (i, dir) in DIRS.iter().enumerate() {
            let new_point = current + *dir;

            if new_point.x < 0 || new_point.y < 0 || new_point.x >= max_x || new_point.y >= max_y {
                continue;
            }

            if visited.contains(&new_point) {
                continue;
            }

            let ch = grid[new_point.y as usize][new_point.x as usize];
            if ch == b'.'
                || ch == b'^' && i == 0
                || ch == b'v' && i == 1
                || ch == b'<' && i == 2
                || ch == b'>' && i == 3
            {
                visited.insert(new_point);
                points.push_back((dist + 1, new_point, visited.clone()));
            }
        }
    }
    *dists.iter().max().unwrap() as u32
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
