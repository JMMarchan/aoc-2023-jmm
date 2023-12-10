use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day08.txt").expect("Day 8 input file should be present.");
    let input: Vec<&str> = input.lines().collect();
    let sol1: u64 = steps_to_exit(&input);
    let sol2: u64 = steps_to_exit_multiple_starts(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

// The input is a list of instructions for map on a network of nodes
// The first line is the sequence of L or R. At each node, we either go R for right, L for left.
// After an empty line is the list of nodes, each with a name and the left and right nodes
// So for example, if we have a line AAA = (BBB, CCC), then AAA is the name of the node, and BBB and CCC are the names of the left and right nodes, respectively
// If we run out of instructions without reaching the end, we loop back to the beginning of instructions, so the instructions are infinite, ie LRR is actually LRRLRRLRR...
// We want to find the number of steps to exit the network, where the start node is AAA and the end node is ZZZ
fn steps_to_exit(input: &[&str]) -> u64 {
    let instructions = input[0].chars().collect::<Vec<_>>();
    let nodes: Vec<(String, String, String)> = parse_nodes(&input[2..]);
    compute_steps_with_conditions("AAA", &instructions, &nodes, |node| node == "ZZZ")
}

fn parse_nodes(input: &[&str]) -> Vec<(String, String, String)> {
    input
        .iter()
        .map(|line| {
            let mut line = line.split(" = ");
            let name = line.next().unwrap();
            let rest = line.next().unwrap().trim_matches(|c| c == '(' || c == ')');
            let mut rest = rest.split(", ");
            let left = rest.next().unwrap();
            let right = rest.next().unwrap();
            // println!("name: {:?}, left: {:?}, right: {:?}", name, left, right);
            (name.to_string(), left.to_string(), right.to_string())
        })
        .collect::<Vec<_>>()
}

// Given a node, a list of instructions, a list of nodes, and a stop condition, find the number of steps it takes to exit the network
fn compute_steps_with_conditions<F>(
    node: &str,
    instructions: &[char],
    nodes: &[(String, String, String)],
    stop: F,
) -> u64
where
    F: Fn(&str) -> bool,
{
    let mut current_node = node;
    let mut steps: usize = 0;
    loop {
        let node = nodes
            .iter()
            .find(|(name, _, _)| name == current_node)
            .unwrap();
        let (_, left, right) = node;
        let next_node = if instructions[steps % instructions.len()] == 'L' {
            left
        } else {
            right
        };
        if stop(next_node) {
            steps += 1;
            break;
        }
        current_node = next_node;
        steps += 1;
    }

    steps as u64
}

// In the second part, we note that there the number of nodes that end with the letter A and the letter Z are the same.
// We are a ghost that exists at multiple nodes at once, and we want to find the number of steps it takes to exit the network.
// So for example, if we have 11A = (11B, XXX) and 22A = (22B, XXX), then we start at 11A and 22A, and the first instruction is L, so we go to 11B and 22B.
// We want to find the number of steps such that every version of us exits the network at a node ending with Z.
// This cannot be brute-forced, so we need to find a way to calculate it.
// Finding the smallest number of steps such that all nodes exit the network at the same time is equivalent to finding the least common multiple of the number of steps it takes for each node to exit the network.
fn steps_to_exit_multiple_starts(input: &[&str]) -> u64 {
    let instructions = input[0].chars().collect::<Vec<_>>();
    let nodes = parse_nodes(&input[2..]);
    let starting_nodes = nodes
        .iter()
        .filter(|(name, _, _)| name.ends_with('A'))
        .collect::<Vec<_>>();

    let path_lengths = starting_nodes
        .iter()
        .map(|&(name, _, _)| {
            compute_steps_with_conditions(name, &instructions, &nodes, |node| node.ends_with('Z'))
        })
        .collect::<Vec<_>>();

    let lcm = path_lengths
        .iter()
        .fold(1, |lcm, &length| lcm * length / gcd(lcm, length));

    lcm
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day08_test_input() -> &'static str {
        "RL\n\
\n\
AAA = (BBB, CCC)\n\
BBB = (DDD, EEE)\n\
CCC = (ZZZ, GGG)\n\
DDD = (DDD, DDD)\n\
EEE = (EEE, EEE)\n\
GGG = (GGG, GGG)\n\
ZZZ = (ZZZ, ZZZ)"
    }

    fn day08_test_input_multiple_starts() -> &'static str {
        "LR\n\
        \n\
11A = (11B, XXX)\n\
11B = (XXX, 11Z)\n\
11Z = (11B, XXX)\n\
22A = (22B, XXX)\n\
22B = (22C, 22C)\n\
22C = (22Z, 22Z)\n\
22Z = (22B, 22B)\n\
XXX = (XXX, XXX)\n\
"
    }

    #[test]
    fn test_steps_to_exit() {
        let input = day08_test_input();
        // split into lines
        let input: Vec<&str> = input.lines().collect();
        assert_eq!(steps_to_exit(&input), 2);
    }

    #[test]
    fn test_steps_to_exit_multiple_starts() {
        let input = day08_test_input_multiple_starts();
        // split into lines
        let input: Vec<&str> = input.lines().collect();
        assert_eq!(steps_to_exit_multiple_starts(&input), 6);
    }
}
