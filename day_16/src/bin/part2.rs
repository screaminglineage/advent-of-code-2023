use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    ops::Add,
};

use itertools::Itertools;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part2(&input);
    println!("part 2: {output}");
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Directions {
    Up = 0,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
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
    Point { x, y }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Ray {
    point: Point,
    direction: Directions,
}

use Directions::*;

fn create_mappings() -> Mapping {
    HashMap::from([
        (
            '|',
            HashMap::from([
                (Up, vec![Up]),
                (Down, vec![Down]),
                (Left, vec![Up, Down]),
                (Right, vec![Up, Down]),
            ]),
        ),
        (
            '-',
            HashMap::from([
                (Up, vec![Left, Right]),
                (Down, vec![Left, Right]),
                (Left, vec![Left]),
                (Right, vec![Right]),
            ]),
        ),
        (
            '/',
            HashMap::from([
                (Up, vec![Right]),
                (Down, vec![Left]),
                (Left, vec![Down]),
                (Right, vec![Up]),
            ]),
        ),
        (
            '\\',
            HashMap::from([
                (Up, vec![Left]),
                (Down, vec![Right]),
                (Left, vec![Up]),
                (Right, vec![Down]),
            ]),
        ),
    ])
}

const DELTAS: [Point; 4] = [new_pt(0, -1), new_pt(0, 1), new_pt(-1, 0), new_pt(1, 0)];
type Mapping = HashMap<char, HashMap<Directions, Vec<Directions>>>;

fn get_energised_tiles(grid: &[Vec<char>], mut rays: VecDeque<Ray>, mappings: &Mapping) -> u32 {
    let mut energized = HashSet::new();
    let max_y = grid.len() as i32;
    let max_x = grid[0].len() as i32;

    while !rays.is_empty() {
        // unwrapping as ray cannot be empty in this loop
        let mut ray = rays.pop_front().unwrap();

        // out of bounds
        if ray.point.y >= max_y || ray.point.x >= max_x || ray.point.y < 0 || ray.point.x < 0 {
            continue;
        }

        if energized.contains(&ray) {
            continue;
        }
        energized.insert(ray);

        let current = grid[ray.point.y as usize][ray.point.x as usize];
        if current == '.' {
            match ray.direction {
                Up => ray.point.y -= 1,
                Down => ray.point.y += 1,
                Left => ray.point.x -= 1,
                Right => ray.point.x += 1,
            }

            rays.push_back(ray);
            continue;
        }

        // need to change direction of ray
        let dirs = mappings.get(&current).unwrap().get(&ray.direction).unwrap();
        for dir in dirs {
            let delta = DELTAS[*dir as usize];
            let new_ray = Ray {
                point: ray.point + delta,
                direction: *dir,
            };
            rays.push_front(new_ray);
        }
    }
    energized.iter().map(|ray| ray.point).unique().count() as u32
}

fn part2(data: &str) -> u32 {
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let mappings: Mapping = create_mappings();

    let mut max_energised = 0;

    // top row
    for i in 0..grid[0].len() {
        let mut rays: VecDeque<Ray> = VecDeque::new();
        rays.push_back(Ray {
            point: Point { y: 0, x: i as i32 },
            direction: Down,
        });
        let a = get_energised_tiles(&grid, rays, &mappings);
        max_energised = max_energised.max(a);
    }

    // bottom row
    for i in 0..grid[0].len() {
        let mut rays: VecDeque<Ray> = VecDeque::new();
        rays.push_back(Ray {
            point: Point {
                y: (grid.len() - 1) as i32,
                x: i as i32,
            },
            direction: Up,
        });
        let a = get_energised_tiles(&grid, rays, &mappings);
        max_energised = max_energised.max(a);
    }

    // left column
    for i in 0..grid.len() {
        let mut rays: VecDeque<Ray> = VecDeque::new();
        rays.push_back(Ray {
            point: Point { y: i as i32, x: 0 },
            direction: Right,
        });
        let a = get_energised_tiles(&grid, rays, &mappings);
        max_energised = max_energised.max(a);
    }

    // right column
    for i in 0..grid.len() {
        let mut rays: VecDeque<Ray> = VecDeque::new();
        rays.push_back(Ray {
            point: Point {
                y: i as i32,
                x: (grid[0].len() - 1) as i32,
            },
            direction: Left,
        });
        let a = get_energised_tiles(&grid, rays, &mappings);
        max_energised = max_energised.max(a);
    }

    max_energised
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

        assert_eq!(output, 51);
    }
}
