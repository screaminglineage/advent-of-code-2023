use std::fs;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

fn expand_rows(space: &mut Vec<Vec<char>>) {
    let mut count = 0;
    loop {
        if count >= space.len() - 1 {
            break;
        }

        let row = &space[count];
        if row.iter().all(|&ch| ch == '.') {
            let new_row = vec!['.'; row.len()];
            if count < row.len() {
                space.insert(count + 1, new_row);
            } else {
                space.push(new_row);
            }
            count += 1;
        }
        count += 1;
    }
}

fn expand_columns(space: &mut Vec<Vec<char>>) {
    let mut col = 0;
    let max_row = space.len();
    loop {
        if col >= space[0].len() - 1 {
            break;
        }

        let mut will_expand = true;
        for i in 0..max_row {
            if space[i][col] == '#' {
                will_expand = false;
                break;
            }
        }
        if will_expand {
            for i in 0..max_row {
                if i < max_row {
                    space[i].insert(col + 1, '.');
                } else {
                    space[i].push('.');
                }
            }
            col += 1;
        }
        col += 1;
    }
}

fn part1(data: &str) -> u32 {
    let mut space: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    expand_rows(&mut space);
    expand_columns(&mut space);

    for (i, row) in space.iter().enumerate() {
        println!("{i} - {row:?}");
    }
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

        assert_eq!(output, 374);
    }
}
