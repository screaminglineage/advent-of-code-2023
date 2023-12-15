use std::fs;

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part2(&input);
    println!("part 2: {output}");
}

#[derive(Copy, Clone, Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_len: u32,
}

fn calculate_hash(seq: &str) -> u32 {
    let mut curr = 0;
    for ch in seq.chars() {
        curr += ch as u32;
        curr *= 17;
        curr %= 256;
    }
    curr
}

const VAL: Vec<Lens> = vec![];

fn part2(data: &str) -> u32 {
    let data: Vec<&str> = data.trim().split(',').collect();
    let mut hashmap: [Vec<Lens>; 256] = [VAL; 256];

    for lens_data in data {
        let lens_data_parsed = lens_data.split(['-', '=']).collect::<Vec<&str>>();
        dbg!(&lens_data_parsed);
        let lens = Lens {
            label: lens_data_parsed[0],
            focal_len: lens_data_parsed[1].parse().unwrap_or_default(),
        };

        let hash = calculate_hash(lens.label) as usize;

        if lens_data.contains(&"-") {
            if let Some(i) = hashmap[hash]
                .iter()
                .position(|inner_lens| inner_lens.label == lens.label)
            {
                hashmap[hash].remove(i);
            }
        } else if lens_data.contains(&"=") {
            match hashmap[hash]
                .iter()
                .position(|inner_lens| inner_lens.label == lens.label)
            {
                // unwrapping is safe as we know i already exists
                Some(i) => *hashmap[hash].get_mut(i).unwrap() = lens,
                None => hashmap[hash].push(lens),
            }
        } else {
            unreachable!("only options are - and =");
        }
    }

    dbg!(&hashmap[0]);

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
    fn part2_works() {
        let data = load_file();
        let output = part2(&data);

        assert_eq!(output, 1320);
    }
}
