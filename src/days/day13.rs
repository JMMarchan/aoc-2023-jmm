use crate::{Solution, SolutionPair};
use core::panic;
use rayon::prelude::*;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day13.txt").expect("Day 13 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    let sol1: u64 = sum_pattern_summary(&lines, 0);
    let sol2: u64 = sum_pattern_summary(&lines, 1);

    (Solution::from(sol1), Solution::from(sol2))
}

// This struct represents the number of differences between columns/rows among their lines of reflection
// Eg columns[0] = 1 means that the difference between column 0 and column 1 is 1 character
// columns[1] = 4 means that there are 4 different characters between column 1 and column 2/column 0 and column 3
#[derive(Debug, PartialEq)]
struct DifferenceCount {
    columns: Vec<usize>,
    rows: Vec<usize>,
}

// Check each possible line of reflection between columns and rows, and count how many differences there are between the two sides of the line of reflection
fn compute_difference_count(grid: &[Vec<char>]) -> DifferenceCount {
    let mut columns = vec![0; grid[0].len() - 1];
    let mut rows = vec![0; grid.len() - 1];

    for i in 1..grid[0].len() {
        for k in 1..=i {
            if i + k - 1 >= grid[0].len() {
                break;
            }
            for row in grid {
                if row[i - k] != row[i + k - 1] {
                    columns[i - 1] += 1;
                }
            }
        }
    }

    for i in 1..grid.len() {
        for k in 1..=i {
            if i + k - 1 >= grid.len() {
                break;
            }
            for j in 0..grid[0].len() {
                if grid[i - k][j] != grid[i + k - 1][j] {
                    rows[i - 1] += 1;
                }
            }
        }
    }

    DifferenceCount { columns, rows }
}

// Input is a list of 2d grids of ash (.) and rocks (#), separated by blank lines.
// The grids have reflections among a row or column
// For example, a 10x10 grid might have a line of reflection between column 5 and 6.
// This means that column 5 and 6 are identical, and so are 4 and 7, 3 and 8, etc. Some columns won't have a reflection, but we can just ignore those.
// To summarize your pattern notes, add the number of columns to the left of the vertical line of reflection or 100 times the number of rows above the horizontal line of reflection.
// Find the sum of the pattern summaries of the grids.
fn sum_pattern_summary(input: &[&str], smudges: usize) -> u64 {
    let grids: Vec<Vec<Vec<char>>> = input
        .split(|x| x.is_empty())
        .map(|x| x.iter().map(|y| y.chars().collect()).collect())
        .collect();

    grids
        .iter()
        .map(|grid| {
            let difference_count = compute_difference_count(grid);
            let column_sum: u64 = difference_count
                .columns
                .iter()
                .enumerate()
                .filter(|&(_, &count)| count == smudges)
                .map(|(index, _)| index + 1)
                .sum::<usize>() as u64;
            let row_sum: u64 = difference_count
                .rows
                .iter()
                .enumerate()
                .filter(|&(_, &count)| count == smudges)
                .map(|(index, _)| (index + 1) * 100)
                .sum::<usize>() as u64;

            column_sum + row_sum
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input_sum() -> &'static str {
        r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#
    }

    #[test]
    fn test_pattern_summary_sum() {
        let input = test_input_sum();
        let input = input.lines().collect::<Vec<&str>>();
        assert_eq!(sum_pattern_summary(&input, 0), 405);
    }

    #[test]
    fn test_compute_difference_count() {
        let input = r#"#.#
#.#
#.."#;
        let input: Vec<Vec<char>> = input
            .lines()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect();
        let expected = DifferenceCount {
            columns: vec![3, 2],
            rows: vec![0, 1],
        };
        assert_eq!(compute_difference_count(&input), expected);
    }
}
