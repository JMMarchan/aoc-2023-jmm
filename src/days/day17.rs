use crate::{Solution, SolutionPair};
use hashbrown::{HashMap, HashSet};
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

// Manhattan distance is not a good heuristic because long straight lines are not penalized enough.
// To more accurately represent the cost of long straight lines, we use a slither pattern.
fn heuristic(pos: (usize, usize), goal: (usize, usize), min_len: usize, max_len: usize) -> usize {
    let dx = isize::abs(pos.0 as isize - goal.0 as isize) as usize;
    let dy = isize::abs(pos.1 as isize - goal.1 as isize) as usize;

    let straight_line_distance = dx.max(dy);

    if dx == 0 || dy == 0 {
        // If we're aligned either horizontally or vertically with the goal,
        // calculate the cost using a slither pattern.
        let full_cycles = straight_line_distance / (max_len + min_len);
        let remaining_distance = straight_line_distance % (max_len + min_len);

        let additional_cost = if remaining_distance <= max_len { 1 } else { 2 };

        full_cycles * 2 + additional_cost
    } else {
        // If diagonal, take an average of the two distances as a simple approximation.
        (dx + dy) / 2
    }
}

fn min_heat_loss(input: &str, max_len: usize, min_len: usize) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8))
        .collect::<Matrix<u8>>();

    let start = Crucible {
        position: (0, 0),
        direction: Direction::None,
    };

    let end = (grid.rows - 1, grid.columns - 1);

    let (_path, cost) = astar(
        &start,
        |crucible| successors(&grid, crucible, max_len, min_len),
        |crucible| heuristic(crucible.position, end, min_len, max_len),
        |crucible| crucible.position == end,
    )
    .unwrap();

    // TODO: add some parameter to print the path
    // let mut grid_view = grid.map(|num| char::from_digit(num as u32, 10).unwrap());
    //
    // // Backtrack from the end to the start, marking the path
    // for (index, state) in _path.iter().enumerate().rev() {
    //     let mut current_position = state.position;
    //     if current_position == start.position {
    //         break;
    //     }
    //     let next_state = &_path[index - 1];
    //
    //     // Mark the path
    //     let (di, dj) = state.direction.to_delta();
    //     while current_position != next_state.position {
    //         grid_view[current_position] = match state.direction {
    //             Direction::North => '↑',
    //             Direction::South => '↓',
    //             Direction::East => '→',
    //             Direction::West => '←',
    //             _ => unreachable!(),
    //         };
    //         current_position = (
    //             (current_position.0 as isize - di) as usize,
    //             (current_position.1 as isize - dj) as usize,
    //         );
    //     }
    // }
    //
    // grid_view[start.position] = '•'; // Start position marked with a dot
    //
    // // Print the grid with the path
    // for row in grid_view.iter() {
    //     for &cell in row.iter() {
    //         print!("{}", cell);
    //     }
    //     println!();
    // }

    cost
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
