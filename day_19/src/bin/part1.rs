use std::{collections::HashMap, fs};

const DATA_FILE: &str = "data.txt";

fn main() {
    let input = fs::read_to_string(DATA_FILE).unwrap();
    let output = part1(&input);
    println!("Part 1: {output}");
}

#[derive(Debug)]
struct Rule<'a> {
    category: char,
    op: char,
    num: u32,
    next: &'a str,
}

#[derive(Debug)]
struct WorkflowRules<'a> {
    rules: Vec<Rule<'a>>,
    next: &'a str,
}

type Part = HashMap<char, u32>;

fn apply_workflow(parts: Vec<Part>, workflows: HashMap<&str, WorkflowRules>) -> u32 {
    let mut accepted: Vec<Part> = Vec::new();

    for part in parts {
        dbg!(&part);
        let mut workflow_name = "in";

        loop {
            if workflow_name == "A" {
                accepted.push(part);
                break;
            } else if workflow_name == "R" {
                break;
            }

            let workflow = workflows.get(workflow_name).unwrap();
            for rule in &workflow.rules {
                let applicant = part.get(&rule.category).unwrap();
                match rule.op {
                    '>' if *applicant > rule.num => workflow_name = workflow.next,
                    '<' if *applicant < rule.num => workflow_name = workflow.next,
                    _ => break,
                }
            }
        }
    }

    accepted.iter().map(|part| part.values().sum::<u32>()).sum()
}

fn part1(data: &str) -> u32 {
    let (workflows, parts) = data.split_once("\n\n").unwrap();
    let parts: Vec<Part> = parts
        .lines()
        .map(|line| {
            let parts = line.split('{').last().unwrap().split('}').next().unwrap();
            parts
                .split(',')
                .map(|categ| {
                    let (c, num) = categ.split_once('=').unwrap();
                    (c.chars().next().unwrap(), num.parse().unwrap())
                })
                .collect::<HashMap<char, u32>>()
        })
        .collect();

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
                rules.push(Rule {
                    category,
                    op,
                    num,
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

    apply_workflow(parts, workflows)
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

        assert_eq!(output, 19114);
    }
}
