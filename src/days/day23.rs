use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day22.txt").expect("Day 22 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    let sol1 = 0;
    let sol2 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

//

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#
    }

    #[test]
    fn test_day20() {
        let lines: Vec<&str> = test_input().lines().collect();
    }
}
