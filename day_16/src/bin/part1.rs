use std::{collections::HashSet, fs};

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

enum Directions {
    Up,
    Down,
    Left,
    Right,
}

struct Point {
    y: i32,
    x: i32,
}

struct Ray {
    point: Point,
    direction: Directions,
}

use Directions::*;

fn move_rays(grid: &[Vec<char>], energized: &mut HashSet<Point>, rays: Vec<Ray>) -> Vec<Ray> {
    let max_y = grid.len() as i32;
    let max_x = grid[0].len() as i32;

    let new_rays = Vec::new();

    for mut ray in rays {
        match ray.direction {
            Up => ray.point.y -= 1,
            Down => ray.point.y += 1,
            Left => ray.point.x -= 1,
            Right => ray.point.x += 1,
        }

        // out of bounds
        if ray.point.y >= max_y || ray.point.x >= max_x || ray.point.y < 0 || ray.point.x < 0 {
            continue;
        }

        match ray.direction {
            _ => (),
        }

        new_rays.push(ray);
    }
    new_rays
}

fn part1(data: &str) -> u32 {
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let mut energized: HashSet<Point> = HashSet::new();
    let mut rays: Vec<Ray> = Vec::new();
    rays.push(Ray {
        point: Point { y: 0, x: 0 },
        direction: Right,
    });
    let rays = move_rays(&grid, &mut energized, rays);
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

        assert_eq!(output, 46);
    }
}
