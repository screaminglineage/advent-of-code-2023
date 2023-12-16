use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    ops::Add,
};

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
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

#[derive(Debug, Copy, Clone)]
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

// not really satisfying but still works weirdly enough
fn get_energised_tiles(grid: &[Vec<char>], mut rays: VecDeque<Ray>, mappings: &Mapping) -> u32 {
    let mut energized = HashSet::new();
    let max_y = grid.len() as i32;
    let max_x = grid[0].len() as i32;

    let max = max_x * max_y;

    // Experimentally found number that does the trick if
    // multiplied by the max elements present in grid
    //
    // Might not work for inputs larger than 110 x 110, which
    // are this input's dimensions. But increasing the number
    // would probably be fine to deal with larger grids
    let magic_multiplier = 30;

    for _ in 0..max * magic_multiplier {
        let mut ray = match rays.pop_front() {
            Some(ray) => ray,
            None => break,
        };
        // out of bounds
        if ray.point.y >= max_y || ray.point.x >= max_x || ray.point.y < 0 || ray.point.x < 0 {
            continue;
        }

        let current = grid[ray.point.y as usize][ray.point.x as usize];

        energized.insert(ray.point);

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
    energized.len() as u32
}

fn part1(data: &str) -> u32 {
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let mut rays: VecDeque<Ray> = VecDeque::new();
    rays.push_back(Ray {
        point: Point { y: 0, x: 0 },
        direction: Right,
    });

    let mappings: Mapping = create_mappings();
    get_energised_tiles(&grid, rays, &mappings)
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
