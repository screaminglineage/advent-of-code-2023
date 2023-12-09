use std::fs;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part2(&input);
    println!("Part 2: {output}");
}

// Iterative solution
fn solve(mut nums: Vec<i32>) -> i32 {
    let mut nums_diffs: Vec<Vec<i32>> = vec![nums.clone()];
    while !nums.iter().all(|&n| n == 0) {
        nums = nums
            .iter()
            .zip(nums.iter().skip(1))
            .map(|(x, y)| y - x)
            .collect();
        nums_diffs.push(nums.clone());
    }

    for i in (1..nums_diffs.len()).rev() {
        let curr = *nums_diffs[i].first().unwrap();
        let prev = *nums_diffs[i - 1].first().unwrap();
        nums_diffs[i - 1].insert(0, prev - curr);
    }

    *nums_diffs.first().unwrap().first().unwrap()
}

// Alternate solution using recursion
fn solve_alternate(nums: Vec<i32>) -> i32 {
    fn solve_recursive(nums: Vec<i32>) -> Vec<i32> {
        if nums.iter().all(|&n| n == 0) {
            let mut new = nums.clone();
            new.insert(0, 0);
            return new;
        }

        let diffs: Vec<i32> = nums.windows(2).map(|chunk| chunk[1] - chunk[0]).collect();
        let mut new = solve_recursive(diffs.clone());

        new.insert(0, nums.first().unwrap() - *new.first().unwrap());
        new
    }

    *solve_recursive(nums).first().unwrap()
}

fn part2(data: &str) -> i32 {
    data.lines()
        .map(|line| line.split(' ').map(|n| n.parse().unwrap()).collect())
        .map(solve_alternate)
        .sum()
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

        assert_eq!(output, 2);
    }
}
