use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day03.txt").expect("Day 3 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    let sol1 = sum_of_part_numbers(&lines);
    let sol2 = sum_of_gear_ratios(&lines);

    (Solution::from(sol1), Solution::from(sol2))
}

struct Schematic {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<char>>,
}

impl Schematic {
    fn new(grid: Vec<Vec<char>>) -> Schematic {
        let rows = grid.len();
        let cols = grid[0].len();
        Schematic { rows, cols, grid }
    }

    fn part_numbers(&self) -> Vec<u32> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| self.process_row(i, row))
            .collect()
    }

    fn process_row(&self, row_idx: usize, row: &[char]) -> Vec<u32> {
        let mut result = Vec::new();
        let mut j = 0;
        while j < row.len() {
            if row[j].is_digit(10) {
                let start = j;
                while j < row.len() && row[j].is_digit(10) {
                    j += 1;
                }
                let end = j;

                if self.is_valid_part_number(row_idx, start, end) {
                    if let Ok(number) = row[start..end].iter().collect::<String>().parse::<u32>() {
                        result.push(number);
                    }
                }
            } else {
                j += 1;
            }
        }
        result
    }

    fn is_valid_part_number(&self, row: usize, start_col: usize, end_col: usize) -> bool {
        for i in (row.max(1) - 1)..=(row + 1).min(self.rows - 1) {
            for j in (start_col.max(1) - 1)..=(end_col).min(self.cols - 1) {
                if self.is_valid_neighbor(row, i, j, start_col, end_col) {
                    return true;
                }
            }
        }
        false
    }

    fn is_valid_neighbor(
        &self,
        row: usize,
        i: usize,
        j: usize,
        start_col: usize,
        end_col: usize,
    ) -> bool {
        (i != row || j < start_col || j >= end_col)
            && self
                .grid
                .get(i)
                .and_then(|r| r.get(j))
                .map_or(false, |&ch| !ch.is_digit(10) && ch != '.')
    }

    fn gear_ratios(&self) -> Vec<u32> {
        let mut ratios = Vec::new();
        for (i, row) in self.grid.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                if ch == '*' {
                    let part_numbers = self.find_adjacent_part_numbers(i, j);
                    // println!("Gear at ({}, {}): {:?}", i, j, part_numbers);
                    if part_numbers.len() == 2 {
                        ratios.push(part_numbers[0].0 * part_numbers[1].0); // Multiply the part numbers
                    }
                }
            }
        }
        ratios
    }

    fn find_adjacent_part_numbers(&self, row: usize, col: usize) -> Vec<(u32, usize, usize)> {
        let mut part_numbers = Vec::new();
        let mut checked_positions = std::collections::HashSet::new();

        // Check each of the eight directions
        for &(dy, dx) in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        {
            let (new_row, new_col) = (row as isize + dy, col as isize + dx);
            if new_row < 0
                || new_row >= self.rows as isize
                || new_col < 0
                || new_col >= self.cols as isize
            {
                continue;
            }

            let (new_row, new_col) = (new_row as usize, new_col as usize);
            if !self.grid[new_row][new_col].is_digit(10) {
                continue;
            }

            let part_number_info = self.get_horizontal_number_at(new_row, new_col);
            if let Some((number, start_col)) = part_number_info {
                let position = (new_row, start_col);
                if checked_positions.contains(&position) {
                    continue;
                }

                part_numbers.push((number, new_row, start_col));
                checked_positions.insert(position);
            }
        }

        part_numbers
    }

    fn get_horizontal_number_at(&self, row: usize, col: usize) -> Option<(u32, usize)> {
        if !self.grid[row][col].is_digit(10) {
            return None;
        }

        let start = self.grid[row][..col]
            .iter()
            .rposition(|&ch| !ch.is_digit(10))
            .map_or(0, |p| p + 1);
        let end = self.grid[row][col..]
            .iter()
            .position(|&ch| !ch.is_digit(10))
            .map_or(self.cols, |p| col + p);

        self.grid[row][start..end]
            .iter()
            .collect::<String>()
            .parse::<u32>()
            .ok()
            .map(|number| (number, start))
    }
}

// The input is a 2d grid of characters, either periods (.), digits, or any other symbol. Digits are guaranteed to be a horizontal sequence.
// A part number is a horizontal sequence of digits such that there exists a non-digit non-period symbol anywhere around it (including diagonally).
// For example
// 123..
// ...@.
// 54...
// 123 is a part number but 54 is not.
// Find the sum of all part numbers.
// Need to find all the numbers in each row.
// Then we can check if each number is a part number by checking its surroundings.
// Then we can sum the part numbers.
fn sum_of_part_numbers(input: &[&str]) -> u32 {
    let grid = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let schematic = Schematic::new(grid);
    let part_numbers = schematic.part_numbers();
    part_numbers.iter().sum::<u32>()
}

// This time, a gear is an asterisk symbol "*" which is next to exactly two part numbers.
// Its gear ratio is the product of the two part numbers.
// Find the sum of all gear ratios.
fn sum_of_gear_ratios(input: &[&str]) -> u32 {
    let grid = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let schematic = Schematic::new(grid);
    let gear_ratios = schematic.gear_ratios();
    gear_ratios.iter().sum::<u32>()
}
