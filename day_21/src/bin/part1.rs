use std::collections::HashSet;
use std::fs;
use std::iter;
use std::ops::Add;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

fn new_pt(x: i32, y: i32) -> Point {
    Point { y, x }
}

fn start_pt(grid: &[Vec<u8>]) -> Point {
    for (y, row) in grid.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            if col == 'S' as u8 {
                return Point {
                    x: x as i32,
                    y: y as i32,
                };
            }
        }
    }
    unreachable!("should have a starting point")
}

fn part1(data: &str) -> u32 {
    let grid: Vec<Vec<u8>> = data.lines().map(|line| line.bytes().collect()).collect();

    let start = start_pt(&grid);
    let directions: [Point; 4] = [new_pt(0, -1), new_pt(0, 1), new_pt(-1, 0), new_pt(1, 0)];

    let mut visited: Vec<HashSet<Point>> = Vec::new();
    visited.push(HashSet::from_iter(iter::once(start)));

    let max_y = grid.len() as i32;
    let max_x = grid[0].len() as i32;
    let max_steps = 64;

    for step in 0..max_steps {
        let current_step = visited[step].clone();
        for point in current_step {
            for dir in directions {
                let new_pt = point + dir;

                if new_pt.x < 0 || new_pt.y < 0 || new_pt.x >= max_x || new_pt.y >= max_y {
                    continue;
                }

                if grid[new_pt.y as usize][new_pt.x as usize] == '#' as u8 {
                    continue;
                }

                match visited.get_mut(step + 1) {
                    Some(points) => {
                        points.insert(new_pt);
                    }
                    None => visited.push(HashSet::from_iter(iter::once(new_pt))),
                }
            }
        }
    }

    visited.last().unwrap().len() as u32
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

        assert_eq!(output, 16);
    }
}
