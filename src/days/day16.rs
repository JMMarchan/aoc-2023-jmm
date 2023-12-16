use crate::{Solution, SolutionPair};
use grid::*;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::VecDeque;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day16.txt").expect("Day 16 input file should be present");
    // Grid takes in a 1d vec, so i need to find the index of the newline to get the number of columns, then filter out the newlines
    let grid_vec: Vec<char> = input.chars().filter(|&c| c != '\n').collect();
    let cols = input.find('\n').unwrap();
    let input = Grid::from_vec(grid_vec.clone(), cols);
    let sol1: u64 = energized_tiles(input, 0, 0, Direction::Right);

    let input = Grid::from_vec(grid_vec, cols);
    let sol2: u64 = max_energized_tiles(input);

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
fn energized_tiles(input: Grid<char>, i: usize, j: usize, direction: Direction) -> u64 {
    let mut seen_beam: HashSet<(usize, usize, Direction)> = HashSet::new();
    let mut energized_tiles: HashSet<(usize, usize)> = HashSet::new();
    let mut current_beam: VecDeque<(usize, usize, Direction)> = VecDeque::new();
    current_beam.push_back((i, j, direction));

    while let Some((i, j, direction)) = current_beam.pop_front() {
        if i >= input.cols() || j >= input.rows() {
            continue;
        }

        if seen_beam.contains(&(i, j, direction)) {
            continue;
        }
        seen_beam.insert((i, j, direction));
        energized_tiles.insert((i, j));

        if let Some(tile) = input.get(i, j) {
            match tile {
                '.' => current_beam.push_back(direction.next_position(i, j)),
                '/' => current_beam.push_back(direction.reflect_slash().next_position(i, j)),
                '\\' => current_beam.push_back(direction.reflect_backslash().next_position(i, j)),
                '|' => current_beam.extend(direction.split_vertical(i, j)),
                '-' => current_beam.extend(direction.split_horizontal(i, j)),
                _ => continue,
            };
        }
    }

    // for reference, create a new grid where each tile is replaced with # if it's energized and . if it's not
    // let mut energized_grid = input.clone();
    // energized_grid.fill('.');
    // for (i, j, _) in seen_beam {
    //     // energized_grid.set(i, j, '#');  // no set method in grid, have to take an iter
    //     energized_grid[(i, j)] = '#';
    // }
    //
    // energized_grid.iter_rows().for_each(|row| {
    //     row.for_each(|tile| print!("{}", tile));
    //     println!();
    // });

    energized_tiles.len() as u64
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next_position(&self, i: usize, j: usize) -> (usize, usize, Self) {
        match self {
            Direction::Up => (i.checked_sub(1).unwrap_or(usize::MAX), j, Direction::Up),
            Direction::Down => (i + 1, j, Direction::Down),
            Direction::Left => (i, j.checked_sub(1).unwrap_or(usize::MAX), Direction::Left),
            Direction::Right => (i, j + 1, Direction::Right),
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

// Find the laser beam entry position that maximizes the number of energized tiles.
fn max_energized_tiles(input: Grid<char>) -> u64 {
    // The possible positions are down from the top row, up from the bottom row,
    // left from the left column, and right from the right column.
    let possible_entries = (0..input.cols())
        .map(|j| (0, j, Direction::Down))
        .chain((0..input.cols()).map(|j| (input.rows() - 1, j, Direction::Up)))
        .chain((0..input.rows()).map(|i| (i, 0, Direction::Right)))
        .chain((0..input.rows()).map(|i| (i, input.cols() - 1, Direction::Left)))
        .collect_vec();

    // The grid is ~100x100, which means perimeter = 400. Run time for one energized_tiles() is ~15ms
    // This would take 6 seconds to run through all 400 positions, which is perfectly fine.
    // Using rayon, only took ~125ms, can certainly opti
    possible_entries
        .par_iter()
        .map(|entry| {
            let (i, j, direction) = entry;
            energized_tiles(input.clone(), *i, *j, *direction)
        })
        .max()
        .unwrap()
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
        assert_eq!(energized_tiles(input, 0, 0, Direction::Right), 46);
    }

    #[test]
    fn test_max_energized_tiles() {
        let input = test_input();
        let grid_vec: Vec<char> = input.chars().filter(|&c| c != '\n').collect();
        let cols = input.find('\n').unwrap();
        let input: Grid<char> = Grid::from_vec(grid_vec, cols);
        assert_eq!(max_energized_tiles(input), 51);
    }
}
