use std::{collections::HashMap, fs, ops::Range};

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part2(&input);
    println!("Part 2: {output}");
}

#[derive(Debug)]
struct Rule<'a> {
    category: char,
    range: Range<u32>,
    next: &'a str,
}

#[derive(Debug)]
struct WorkflowRules<'a> {
    rules: Vec<Rule<'a>>,
    next: &'a str,
}

type Part = HashMap<char, Range<u32>>;

fn apply_workflow(mut part: Part, workflows: HashMap<&str, WorkflowRules>) -> Vec<Range<u32>> {
    let mut workflow_name = "in";

    let mut result = Vec::new();

    loop {
        if workflow_name == "A" {
            break;
        } else if workflow_name == "R" {
            continue;
        }

        let workflow = workflows.get(workflow_name).unwrap();

        for rule in &workflow.rules {
            let part_range = part.get(&rule.category).unwrap();

            match (
                part_range.start < rule.range.start,
                part_range.end < rule.range.end,
            ) {
                (true, true) => result.push(rule.range.start..(part_range.end + 1)),
                (false, true) => {
                    result.push(part_range.start..(part_range.end - rule.range.end + 1))
                }
                (true, false) => {
                    result.push(rule.range.start..(part_range.end - rule.range.end + 1))
                }
                (false, false) => result.push(part_range.start..(rule.range.end + 1)),
            }
            workflow_name = workflow.next;
        }
    }

    result
}

fn part2(data: &str) -> u64 {
    let (workflows, _) = data.split_once("\n\n").unwrap();

    let parts = HashMap::from([
        ('x', 1..(4000 + 1)),
        ('m', 1..(4000 + 1)),
        ('a', 1..(4000 + 1)),
        ('s', 1..(4000 + 1)),
    ]);

    let workflows: HashMap<&str, WorkflowRules> = workflows
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once('{').unwrap();
            let rest: Vec<&str> = rest.split('}').next().unwrap().split(',').collect();

            let mut rules = Vec::new();
            for rule in rest.iter().take(rest.len() - 1) {
                let (rule, next) = rule.split_once(':').unwrap();
                let mut rule_it = rule.chars();

                let category = rule_it.next().unwrap();
                let op = rule_it.next().unwrap();
                let num = rule_it.collect::<String>().parse::<u32>().unwrap();

                let range = match op {
                    '>' => (num + 1)..(4000 + 1),
                    '<' => 1..num,
                    _ => unreachable!(),
                };
                rules.push(Rule {
                    category,
                    range,
                    next,
                });
            }
            let next_workflow = rest.last().unwrap();
            (
                name,
                WorkflowRules {
                    rules,
                    next: next_workflow,
                },
            )
        })
        .collect();

    dbg!(apply_workflow(parts, workflows));

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

        assert_eq!(output, 167409079868000);
    }
}
