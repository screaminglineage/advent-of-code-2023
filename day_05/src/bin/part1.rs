use std::{collections::HashMap, fs, ops::Range};

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

#[derive(Debug)]
struct RangeMap {
    source: Range<u64>,
    dest: Range<u64>,
}

fn range_from_nums(nums: Vec<u64>) -> Vec<RangeMap> {
    nums.chunks(3)
        .map(|num| {
            let dest_start = num[0];
            let source_start = num[1];
            let length = num[2];
            RangeMap {
                source: Range {
                    start: source_start,
                    end: source_start + length,
                },

                dest: Range {
                    start: dest_start,
                    end: dest_start + length,
                },
            }
        })
        .collect::<Vec<RangeMap>>()
}

fn get_ranges(line: &str) -> Vec<RangeMap> {
    let num_ranges: Vec<u64> = line
        .split('\n')
        .flat_map(|nums| nums.split_whitespace().map(|n| n.parse::<u64>().unwrap()))
        .collect();
    range_from_nums(num_ranges)
}

fn get_seeds(data: &[&str]) -> Vec<u64> {
    data.first()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn map_items(item: u64, maps: &HashMap<&str, Vec<RangeMap>>, map: &str) -> u64 {
    let map_ranges = maps.get(map).unwrap();

    if let Some(range) = map_ranges.iter().find(|range| range.source.contains(&item)) {
        (item as i64 - (range.source.start as i64 - range.dest.start as i64)) as u64
    } else {
        item
    }
}

fn part1(data: &str) -> u64 {
    let mut data_lines: Vec<&str> = data.split("\n\n").collect();
    let seeds = get_seeds(&data_lines);
    let data_lines: Vec<&str> = data_lines.drain(1..).collect();

    let mut maps: HashMap<&str, Vec<RangeMap>> = HashMap::new();
    for line in data_lines {
        let map_name = line.split(":\n").next().unwrap();
        let ranges = line.split(":\n").last().unwrap();
        let ranges = get_ranges(ranges);
        maps.insert(map_name, ranges);
    }
    let map_order = vec![
        "seed-to-soil map",
        "soil-to-fertilizer map",
        "fertilizer-to-water map",
        "water-to-light map",
        "light-to-temperature map",
        "temperature-to-humidity map",
        "humidity-to-location map",
    ];

    seeds
        .iter()
        .map(|seed| {
            let mut location = *seed;
            for map in &map_order {
                location = map_items(location, &maps, map);
            }
            location
        })
        .min()
        .unwrap()
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

        assert_eq!(output, 35);
    }
}
