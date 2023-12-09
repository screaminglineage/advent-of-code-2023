use std::fs;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

fn part1(data: &str) -> i32 {
    data.lines()
        .map(|line| {
            let mut nums: Vec<i32> = line.split(" ").map(|n| n.parse().unwrap()).collect();

            let mut prev_nums: Vec<Vec<i32>> = vec![nums.clone()];
            while !nums.iter().all(|&n| n == 0) {
                nums = nums
                    .iter()
                    .zip(nums.iter().skip(1))
                    .map(|(x, y)| y - x)
                    .collect();
                prev_nums.push(nums.clone());
            }

            for i in (1..prev_nums.len()).rev() {
                let curr = *prev_nums[i].last().unwrap();
                let prev = *prev_nums[i - 1].last().unwrap();
                prev_nums[i - 1].push(curr + prev);
            }

            *prev_nums.iter().next().unwrap().last().unwrap()
        })
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
    fn part1_works() {
        let data = load_file();
        let output = part1(&data);

        assert_eq!(output, 114);
    }
}
