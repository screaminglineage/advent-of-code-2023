use std::fs;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part2(&input);
    println!("Part 2: {output}");
}

fn is_symbol(ch: char) -> bool {
    !ch.is_numeric() && ch != '.'
}

fn is_schematic(schema: &[Vec<char>], row: usize, col: usize) -> bool {
    let mut neighbours = Vec::new();
    for i in -1i32..2i32 {
        for j in -1i32..2i32 {
            let new_row = row as i32 + i;
            let new_col = col as i32 + j;

            if new_row < 0 || new_col < 0 {
                continue;
            }
            if new_row >= schema.len() as i32 || new_col >= schema[0].len() as i32 {
                continue;
            }
            neighbours.push(schema[new_row as usize][new_col as usize]);
        }
    }

    println!("{neighbours:?}");
    neighbours.iter().any(|c| is_symbol(*c))
}

fn add_to_sum(sum: &mut u32, num: &[(usize, usize, &char)], schema: &[Vec<char>]) {
    if num.iter().any(|(x, y, _)| is_schematic(&schema, *x, *y)) {
        let s = num.iter().map(|(_, _, c)| **c).collect::<String>();
        *sum += s.parse::<u32>().unwrap();
    }
}

// OOF this sucks so much
fn part2(data: &str) -> u32 {
    let schema: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    for (row, line) in schema.iter().enumerate() {
        let mut num = Vec::new();
        for (col, ch) in line.iter().enumerate() {
            if ch.is_numeric() {
                num.push((row, col, ch));
            } else {
                add_to_sum(&mut sum, &num, &schema);
                num.clear();
            }
            if col == schema[0].len() - 1 && !num.is_empty() {
                add_to_sum(&mut sum, &num, &schema);
                num.clear();
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
    fn part1_works() {
        let data = load_file();
        let output = part2(&data);

        assert_eq!(output, 4361);
    }

    #[test]
    fn symbol() {
        assert!(is_symbol('+'));
        assert!(!is_symbol('.'));
        assert!(!is_symbol('6'));
    }
}
