use crate::{Solution, SolutionPair};
use std::{collections::HashSet, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day11.txt").expect("Day 11 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    let sol1: u64 = sum_pairwise_space_distances(&lines, 2);
    let sol2: u64 = sum_pairwise_space_distances(&lines, 1_000_000);

    (Solution::from(sol1), Solution::from(sol2))
}

// The input is a 2d grid of characters with empty space (.) and galaxies (#)
// We need to get the distance of the shortest path between all pairs of galaxies on this grid (ie the shortest path between them)
// However, some space expands. In particular, a row of empty space counts as n rows of empty space, and a column of empty space counts as n columns of empty space.
// In the first part, n is 2. In the second part, n is 1,000,000.
fn sum_pairwise_space_distances(input: &[&str], expansion_factor: u64) -> u64 {
    let grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let mut sum = 0;

    // Count expanded rows and columns
    let expanded_rows: HashSet<usize> = (0..grid.len())
        .filter(|&i| grid[i].iter().all(|&c| c == '.'))
        .collect();
    let expanded_columns: HashSet<usize> = (0..grid[0].len())
        .filter(|&i| grid.iter().all(|row| row[i] == '.'))
        .collect();

    // Collect galaxy coordinates
    let galaxy_coords: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(
                move |(j, &col)| {
                    if col == '#' {
                        Some((i, j))
                    } else {
                        None
                    }
                },
            )
        })
        .collect();

    // Calculate distances
    for i in 0..galaxy_coords.len() {
        for j in i + 1..galaxy_coords.len() {
            let (x1, y1) = galaxy_coords[i];
            let (x2, y2) = galaxy_coords[j];
            let mut distance = 0;

            // Calculate row distance
            distance += calculate_distance(x1, x2, &expanded_rows, expansion_factor);

            // Calculate column distance
            distance += calculate_distance(y1, y2, &expanded_columns, expansion_factor);

            sum += distance;
        }
    }

    sum
}

// Helper function to calculate distance considering expanded rows or columns
fn calculate_distance(
    start: usize,
    end: usize,
    expanded: &HashSet<usize>,
    expansion_factor: u64,
) -> u64 {
    let mut distance = 0;
    let range_start = start.min(end) + 1;
    let range_end = start.max(end);

    for k in range_start..=range_end {
        distance += if expanded.contains(&k) {
            expansion_factor
        } else {
            1
        };
    }

    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#
    }

    fn simple_input() -> &'static str {
        r#"#..
...
..#"#
    }

    #[test]
    fn test_simple_input() {
        let input = simple_input();
        let input: Vec<&str> = input.lines().collect();
        assert_eq!(sum_pairwise_space_distances(&input, 100), 202);
    }

    #[test]
    fn test_sum_pairwise_space_distances() {
        let input = test_input();
        let input: Vec<&str> = input.lines().collect();
        assert_eq!(sum_pairwise_space_distances(&input, 2), 374);
    }

    #[test]
    fn test_sum_pairwise_space_distances_larger() {
        let input = test_input();
        let input: Vec<&str> = input.lines().collect();
        assert_eq!(sum_pairwise_space_distances(&input, 100), 8410);
    }
}
