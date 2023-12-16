use crate::{Solution, SolutionPair};
use grid::*;
use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day16.txt").expect("Day 16 input file should be present");
    // Grid takes in a 1d vec, so i need to find the index of the newline to get the number of columns, then filter out the newlines
    let grid_vec: Vec<char> = input.chars().filter(|&c| c != '\n').collect();
    let cols = input.find('\n').unwrap();
    let input = Grid::from_vec(grid_vec, cols);
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

// Input is a grid with empty space (.), mirrors (/) and (\), and splitters (|) and (-).
// Beam of light starts at top left from left, heading right.
// On empty space, beam continues in same direction.
// At a mirror, the beam is reflected 90 degrees depending on the direction of the mirror,
// eg a right-moving beam hitting / will be reflected upwards.
// If the beam encounters the pointy end of a splitter, the beam passes through the splitter and continues in the same direction.
// If the beam encounters the flat end of a splitter, the beam is split into the two perpendicular directions.
// If the beam encounters the edge of the grid, it ends.
// Beams do not interact with each other.
// A tile is energized if that tile has at least one beam of light passing through it.
// Find the number of energized tiles.
fn energized_tiles(input: Grid<char>) -> u64 {
    println!("grid: {:?}", input);
    let mut seen_beam: HashSet<(usize, usize, Direction)> = HashSet::new();
    let mut current_beam: VecDeque<(usize, usize, Direction)> = VecDeque::new();

    current_beam.push_back((0, 0, Direction::Right));
    let mut energized_tiles = 0;

    // ith row, jth column
    while let Some((i, j, direction)) = current_beam.pop_front() {
        if i >= input.cols() || j >= input.rows() {
            continue;
        }
        println!(
            "i: {}, j: {}, direction: {:?}, tile: {}",
            i,
            j,
            direction,
            input.get(i, j).unwrap()
        );
        // Skip if this beam has been processed before
        if seen_beam.contains(&(i, j, direction)) {
            println!("seen beam");
            continue;
        }
        // Mark this beam as seen and increment the count of energized tiles
        seen_beam.insert((i, j, direction));
        energized_tiles += 1;

        // Determine the next position and direction of the beam
        if let Some(tile) = input.get(i, j) {
            match tile {
                '.' => current_beam.push_back(direction.next_position(i, j)),
                '/' => current_beam.push_back(direction.reflect_slash().next_position(i, j)),
                '\\' => current_beam.push_back(direction.reflect_backslash().next_position(i, j)),
                '|' => current_beam.extend(direction.split_vertical(i, j)),
                '-' => current_beam.extend(direction.split_horizontal(i, j)),
                _ => continue, // Skip unknown tiles
            };
        }
    }

    energized_tiles
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    // Method to calculate the next position based on the current direction
    fn next_position(&self, i: usize, j: usize) -> (usize, usize, Self) {
        match self {
            Direction::Up => (i.checked_sub(1).unwrap_or(usize::MAX), j, *self),
            Direction::Down => (i + 1, j, *self),
            Direction::Left => (i, j.checked_sub(1).unwrap_or(usize::MAX), *self),
            Direction::Right => (i, j + 1, *self),
        }
    }

    fn reflect_slash(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn reflect_backslash(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn split_horizontal(&self, i: usize, j: usize) -> Vec<(usize, usize, Self)> {
        match self {
            Direction::Up | Direction::Down => {
                vec![
                    (i, j.checked_sub(1).unwrap_or(usize::MAX), Direction::Left),
                    (i, j + 1, Direction::Right),
                ]
            }
            Direction::Left | Direction::Right => {
                vec![(self.next_position(i, j))]
            }
        }
    }

    fn split_vertical(&self, i: usize, j: usize) -> Vec<(usize, usize, Self)> {
        match self {
            Direction::Left | Direction::Right => {
                vec![
                    (i.checked_sub(1).unwrap_or(usize::MAX), j, Direction::Up),
                    (i + 1, j, Direction::Down),
                ]
            }
            Direction::Up | Direction::Down => {
                vec![(self.next_position(i, j))]
            }
        }
    }
}

fn _function2(_input: &[&str]) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#
    }

    #[test]
    fn test_energized_tiles() {
        let input = test_input();
        let grid_vec: Vec<char> = input.chars().filter(|&c| c != '\n').collect();
        let cols = input.find('\n').unwrap();
        let input: Grid<char> = Grid::from_vec(grid_vec, cols);
        assert_eq!(energized_tiles(input), 46);
    }

    #[test]
    fn test_function_2() {
        let input = test_input();
        let input: Vec<&str> = input.lines().collect();
        assert_eq!(_function2(&input), 0);
    }
}
