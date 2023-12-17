use crate::{Solution, SolutionPair};
use hashbrown::HashMap;
use pathfinding::prelude::{astar, Matrix};
use rayon::prelude::*;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day17.txt").expect("Day 17 input file should be present");
    let sol1 = min_heat_loss(&input, 3, 1);
    let sol2 = min_heat_loss(&input, 10, 4);

    (Solution::from(sol1), Solution::from(sol2))
}

// Input is a 2d grid of numbers representing the heat loss of each tile.
// Goal is to find path for the crucible to minimize heat loss.
// The start is the top left, and the end is the bottom right.
// The digit on a tile represents the heat loss of that tile when entered,
// so even though you start on the top left, you don't lose heat unless you re-enter the tile.
// The crucible can go at most 3 tiles in a straight line, then it must turn 90 degrees left or right.
// The crucible also can't reverse direction, it can only ever turn left, turn right, or go straight.
// In the second part, the crucible must go at least 4 tiles in a straight line and can go at most 10 tiles in a straight line.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
    None,
}

impl Direction {
    fn to_delta(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
            Direction::None => (0, 0),
        }
    }

    fn perpendicular_directions(&self) -> (Direction, Direction) {
        match self {
            Direction::North | Direction::South => (Direction::East, Direction::West),
            Direction::East | Direction::West => (Direction::North, Direction::South),
            Direction::None => (Direction::South, Direction::East),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Crucible {
    position: (usize, usize),
    direction: Direction,
}

fn successors(
    grid: &Matrix<u8>,
    state: &Crucible,
    max_run_length: usize,
    min_run_length: usize,
) -> Vec<(Crucible, usize)> {
    let mut next_crucibles = Vec::new();
    let (left_turn, right_turn) = state.direction.perpendicular_directions();
    let directions = vec![left_turn, right_turn];

    for dir in directions {
        // Calculate the heat loss moving along the straight line
        let mut heat_loss = 0;
        for run in 1..max_run_length + 1 {
            let (i, j) = state.position;
            let delta = dir.to_delta();
            let next_position =
                grid.move_in_direction((i, j), (delta.0 * run as isize, delta.1 * run as isize));

            if next_position.is_none() {
                break;
            }

            let next_position = next_position.unwrap();

            heat_loss += grid[next_position] as usize;

            if run >= min_run_length {
                next_crucibles.push((
                    Crucible {
                        position: next_position,
                        direction: dir,
                    },
                    heat_loss,
                ));
            }
        }
    }

    next_crucibles
}

fn manhattan_distance(pos: (usize, usize), goal: (usize, usize)) -> usize {
    let dx = isize::abs(pos.0 as isize - goal.0 as isize);
    let dy = isize::abs(pos.1 as isize - goal.1 as isize);
    (dx + dy) as usize
}

fn min_heat_loss(input: &str, max_run_length: usize, min_run_length: usize) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8))
        .collect::<Matrix<u8>>();

    let start = Crucible {
        position: (0, 0),
        direction: Direction::None,
    };

    let end = (grid.rows - 1, grid.columns - 1);

    astar(
        &start,
        |crucible| successors(&grid, crucible, max_run_length, min_run_length),
        |crucible| manhattan_distance(crucible.position, end),
        |crucible| crucible.position == end,
    )
    .map(|(_, cost)| cost)
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input_simple() -> &'static str {
        r#"123
333
111"#
    }

    #[test]
    fn test_min_heat_loss_simple() {
        assert_eq!(min_heat_loss(test_input_simple(), 3, 1), 6);
    }

    fn test_input() -> &'static str {
        r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#
    }

    #[test]
    fn test_min_heat_loss_sample() {
        assert_eq!(min_heat_loss(test_input(), 3, 1), 102);
    }

    #[test]
    fn test_min_heat_loss_ultra() {
        assert_eq!(min_heat_loss(test_input(), 10, 4), 94);
    }
}
