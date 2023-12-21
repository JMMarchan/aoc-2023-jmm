use crate::{Solution, SolutionPair};
use grid::Grid;
use hashbrown::{HashMap, HashSet};
use rayon::prelude::*;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day19.txt").expect("Day 19 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    let sol1 = sum_rating_numbers(&lines);
    let sol2 = possible_rating_numbers(&lines);

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

// Rule represented as a tuple: (category, operator, value, destination)
type Rule = (char, char, usize, String);

// Workflows as a HashMap from label to a list of rules
type Workflows = HashMap<String, Vec<Rule>>;

// Inputs has two line-separated lists, split by a blank line.
// The first list is a set of workflows which accept or reject parts.
// The second list is a set of parts, rated in four categories: x, m, a, and s.
// A workflow starts with a label, followed by rules contained in curly braces.
// The rules are a comma-separated list of > or < conditions on a category rating,
// followed by the destination, which maybe be a label, or "R" for reject, or "A" for accept.
// The final rule has no condition, and is the default destination.
// All parts start at the workflow labeled "in".
// They are then routed through the workflows, following the rules based on their ratings, until they are accepted or rejected.
// Find the sum of all the ratings for all the parts that are accepted.
fn sum_rating_numbers(input: &[&str]) -> usize {
    let mut part_list: Vec<Part> = Vec::new();

    let input_sections: Vec<&[&str]> = input.split(|line| line.is_empty()).collect();
    let workflows = parse_workflows(input_sections[0]);
    let part_lines = input_sections[1];

    // Parse parts
    for line in part_lines {
        let mut x = 0;
        let mut m = 0;
        let mut a = 0;
        let mut s = 0;

        for part in line
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
        {
            let mut parts = part.split('=');
            let category = parts.next().unwrap().trim();
            let value = parts.next().unwrap().trim().parse::<usize>().unwrap();
            match category {
                "x" => x = value,
                "m" => m = value,
                "a" => a = value,
                "s" => s = value,
                _ => unreachable!(),
            }
        }

        part_list.push(Part { x, m, a, s });
    }

    // println!("{:?}", workflows);
    // println!("{:?}", part_list);

    // Implement Workflow Logic
    part_list
        .par_iter()
        .map(|part| {
            let mut current_label = "in";

            while let Some(rules) = workflows.get(current_label) {
                if let Some((_, _, _, dest)) =
                    rules.iter().find(|(category, operator, value, _)| {
                        let rating = match *category {
                            'x' => part.x,
                            'm' => part.m,
                            'a' => part.a,
                            's' => part.s,
                            _ => 0,
                        };
                        match *operator {
                            '>' => rating > *value,
                            '<' => rating < *value,
                            _ => false,
                        }
                    })
                {
                    if dest == "R" {
                        return 0;
                    } else if dest == "A" {
                        return part.x + part.m + part.a + part.s;
                    } else {
                        current_label = dest;
                    }
                }
            }

            // A matching rule will always be found, because the last rule has no condition
            // so its condition is set to x > 0, which is always true
            unreachable!();
        })
        .sum()
}

fn parse_workflows(workflow_lines: &[&str]) -> Workflows {
    let mut workflows: Workflows = HashMap::new();

    for line in workflow_lines {
        let mut parts = line.split('{');
        let label = parts.next().unwrap().trim().to_string();
        let mut rules_str = parts
            .next()
            .unwrap()
            .trim_end_matches('}')
            .split(',')
            .collect::<Vec<_>>();

        // Handle the last rule separately
        let last_rule_str = rules_str.pop().unwrap();
        let last_dest = last_rule_str.trim().to_string();

        // Now, iterate over the remaining rules
        let mut rules: Vec<Rule> = rules_str
            .iter()
            .map(|rule_str| {
                let mut parts = rule_str.split(':');
                let condition = parts.next().unwrap().trim();
                let category = condition.chars().next().unwrap();
                let operator = condition.chars().nth(1).unwrap();
                let value = condition[2..].parse::<usize>().unwrap();
                let dest = parts.next().unwrap().trim();
                (category, operator, value, dest.to_string())
            })
            .collect();

        // Add the last rule (default destination), which has no condition.
        // Give it a condition of x > 0, which is always true so it will always be matched.
        rules.push(('x', '>', 0, last_dest));

        workflows.insert(label, rules);
    }

    workflows
}

// We have the same set of workflows which accept or reject parts.
// Ignore the parts list.
// Each of the ratings can have an integer value from 1 to 4000.
// Find the number of combinations of ratings that are accepted by the workflows.
fn possible_rating_numbers(input: &[&str]) -> u64 {
    // This solution works similarly to Day 5 in taking ranges of numbers and splitting them based on rules.
    let input_sections: Vec<&[&str]> = input.split(|line| line.is_empty()).collect();
    let workflows = parse_workflows(input_sections[0]);
    let mut range_collection: Vec<(RatingRange, &str)> = vec![(RatingRange::new(), "in")];
    let mut accepted_ranges: Vec<RatingRange> = Vec::new();

    while let Some((range, current_label)) = range_collection.pop() {
        // println!("Range: {:?}, Label: {}", range, current_label);
        if current_label == "A" {
            accepted_ranges.push(range);
            continue;
        } else if current_label == "R" {
            continue;
        }

        if let Some(rules) = workflows.get(current_label) {
            // println!("Label: {}, Rules: {:?}", current_label, rules);
            // Find the default destination
            // Apply rules and split/forward ranges
            let mut current_ranges: Vec<RatingRange> = vec![range];

            for rule in rules {
                // Process all but the last rule
                let (category, operator, value, destination) = rule;
                // println!(
                //     "Rule: {} {} {} -> {}",
                //     category, operator, value, destination
                // );
                let mut next_ranges = Vec::new();

                for range in current_ranges {
                    let (mut ranges_to_process, ranges_to_forward) =
                        range.split_at(*category, *operator, *value);
                    next_ranges.append(&mut ranges_to_process);
                    for forward_range in ranges_to_forward {
                        // println!(
                        //     "Forwarding range: {:?}, Destination: {}, Combinations: {}",
                        //     forward_range,
                        //     destination,
                        //     forward_range.combinations()
                        // );
                        range_collection.push((forward_range, &destination));
                    }
                }

                current_ranges = next_ranges;
            }
        }
    }

    // println!("Accepted ranges: {:?}", accepted_ranges);

    // Count valid combinations in accepted_ranges
    accepted_ranges
        .iter()
        .map(|range| {
            // Calculate the product of the lengths of each range
            range.combinations()
        })
        .sum()
}

// A range in [start, end) interval notation
#[derive(Debug, Clone)]
struct RatingRange {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl RatingRange {
    fn new() -> Self {
        RatingRange {
            x: (1, 4001), // using 4001 as the end to represent 1 to 4000
            m: (1, 4001),
            a: (1, 4001),
            s: (1, 4001),
        }
    }

    fn combinations(&self) -> u64 {
        ((self.x.1 - self.x.0)
            * (self.m.1 - self.m.0)
            * (self.a.1 - self.a.0)
            * (self.s.1 - self.s.0)) as u64
    }

    // Split the range based on the rule and return the next ranges and the ranges to forward
    fn split_at(
        &self,
        category: char,
        operator: char,
        value: usize,
    ) -> (Vec<RatingRange>, Vec<RatingRange>) {
        let (start, end) = match category {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!(),
        };

        let mut next_ranges = Vec::new();
        let mut forward_ranges = Vec::new();

        match operator {
            '<' => {
                if start < value {
                    if end > value {
                        // [start, value) gets pushed forward, [value, end) gets processed to the next rule
                        forward_ranges.push(self.with_new_range(category, start, value));
                        next_ranges.push(self.with_new_range(category, value, end));
                    } else {
                        // All of [start, end) gets pushed forward
                        forward_ranges.push(self.clone());
                    }
                } else {
                    // All of [start, end) gets processed to the next rule
                    next_ranges.push(self.clone());
                }
            }
            '>' => {
                if end > value {
                    if start < value {
                        // [start, value) gets processed to the next rule, [value, end) gets pushed forward
                        forward_ranges.push(self.with_new_range(category, value + 1, end));
                        next_ranges.push(self.with_new_range(category, start, value + 1));
                    } else {
                        // All of [start, end) gets pushed forward
                        forward_ranges.push(self.clone());
                    }
                } else {
                    // All of [start, end) gets processed to the next rule
                    next_ranges.push(self.clone());
                }
            }
            _ => unreachable!(),
        }

        (next_ranges, forward_ranges)
    }

    fn with_new_range(&self, category: char, start: usize, end: usize) -> Self {
        let mut new_range = self.clone();
        match category {
            'x' => new_range.x = (start, end),
            'm' => new_range.m = (start, end),
            'a' => new_range.a = (start, end),
            's' => new_range.s = (start, end),
            _ => unreachable!(),
        }
        new_range
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#
    }

    #[test]
    fn test_sum_rating_numbers_sample() {
        let input = test_input().lines().collect::<Vec<&str>>();
        assert_eq!(sum_rating_numbers(&input), 19114);
    }

    #[test]
    fn test_possible_rating_numbers_sample() {
        let input = test_input().lines().collect::<Vec<&str>>();
        assert_eq!(possible_rating_numbers(&input), 167409079868000);
    }
}
