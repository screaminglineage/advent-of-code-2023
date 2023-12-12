#!/bin/bash
# Generates a Rust Template for Advent of Code

# Regular Expression to check if number
IS_NUM="^[0-9]+$"

# Shows information about this command
help() {
    echo "usage: genaoc [day_count]"
    echo ""
    echo "Generates a Rust Template for Advent of Code"
    echo ""
    echo "positional arguments:"
    echo "  day_count      Number of Day for AOC"
    echo ""
    echo "options:"
    echo "  -h, --help  show this help message and exit"
}


gen_aoc() {
    day="day_${1}"
    
    cargo new --vcs none "${day}" || exit
    
    # Deletes pre-created main.rs
    rm "${day}/src/main.rs"

    bin="${day}/src/bin"
    mkdir "$bin"
    part1_rs="${bin}/part1.rs"

    # Creates files to store data
    touch "${day}/test_data.txt"
    touch "${day}/data.txt"
    
    # Creates source files
    echo "use std::fs;

const DATA_FILE: &str = \"data.txt\";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!(\"Part 1: {output}\");
}

fn part1(data: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA_FILE: &str = \"test_data.txt\";

    fn load_file() -> String {
        fs::read_to_string(TEST_DATA_FILE).unwrap()
    }

    #[test]
    fn part1_works() {
        let data = load_file();
        let output = part1(&data);

        assert_eq!(output, \"\");
    }
}" > "$part1_rs"

}    


# Shows help and exits if the argument is -h
if [[ $1 = -h ]] || [[ $1 = --help ]]; then
    help
    exit 0
elif [[ $1 =~ $IS_NUM ]]; then
    gen_aoc "$1" "$2"
else
    echo "Enter a Day Number"
    exit 1
fi


