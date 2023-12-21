use std::fs;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

#[derive(Debug, Clone, Copy)]
struct Point {
    y: i32,
    x: i32,
}

fn start_pt(grid: &[Vec<u8>]) -> Point {
    for (y, row) in grid.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            if col == 'S' as u8 {
                return Point {
                    y: y as i32,
                    x: x as i32,
                };
            }
        }
    }
    unreachable!("should have a starting point")
}

fn part1(data: &str) -> u32 {
    let grid: Vec<Vec<u8>> = data.lines().map(|line| line.bytes().collect()).collect();

    dbg!(start_pt(&grid));

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

        assert_eq!(output, 16);
    }
}
