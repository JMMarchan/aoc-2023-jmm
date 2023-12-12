use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // let input = read_to_string("input/day13.txt").expect("Day 13 input file should be present");
    // let lines: Vec<&str> = input.lines().collect();
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn _function1(input: &[&str]) -> u64 {
    0
}

fn _function2(input: &[&str]) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"line1
line2
line3"#
    }

    #[test]
    fn test_function_1() {
        let input = test_input();
        let input: Vec<&str> = input.lines().collect();
        assert_eq!(_function1(&input), 0);
    }

    #[test]
    fn test_function_2() {
        let input = test_input();
        let input: Vec<&str> = input.lines().collect();
        assert_eq!(_function2(&input), 0);
    }
}
