use crate::{Solution, SolutionPair};
use grid::Grid;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day18.txt").expect("Day 18 input file should be present");
    let _lines: Vec<&str> = input.lines().collect();
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn _function1(_input: &[&str]) -> u64 {
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
        let input = test_input().lines().collect::<Vec<&str>>();
        assert_eq!(_function1(&input), 0);
    }
}
