use crate::{Solution, SolutionPair};
use hashbrown::HashSet;
use itertools::Itertools;
use pathfinding::prelude::{bfs_reach, Matrix};
use std::fs::read_to_string;
use std::hash::Hash;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day21.txt").expect("Day 20 input file should be present");
    let sol1 = num_garden_plots(&input, 64);
    let sol2 = num_garden_plots_infinite_grid(&input, 26501365);

    (Solution::from(sol1), Solution::from(sol2))
}

// The input is a 2d grid of the starting position S, garden plots ., and rocks #.
// The elf can move up, down, left, or right onto a garden plot.
// We need to find how many garden plots the elf can reach in 64 moves.
// This is not just distance, consider the starting position, which is always reachable in 2, 4, 6, etc. steps.
// Or consider any tiles adjacent to the starting position, which is reachable in 1, 3, 5, etc. steps.
fn num_garden_plots(input: &str, max_steps: usize) -> u64 {
    // If a tile is reachable in k steps, then it is reachable in k+2 steps by moving to the adjacent tile and then back.
    // Thus even number distance tiles up to 64 are reachable.
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Matrix<char>>();

    // Start with the starting position S
    // count the number of tiles we've seen
    // We can use BFS, with a successor function of two steps away in each direction and tile not seen before
    // two steps away is (-2,0), (-1,-1), (0,-2), (1,-1), (2,0), (1,1), (0,2), (-1,1)
    // ie the immediate diagonals and the cardinals two tiles away
    // Though we also have to check if each of those neighbors can be reached in two steps
    // For any of the directions, see if there is a rock on that tile or a previously visited tile
    // For the cardinal directions, see if there is a rock between the current and the cardinal tile
    // For the diagonal directions, see if there are two rocks on both of the cardinal tiles between the current and the diagonal tile

    reachable_garden_plots(&grid, max_steps)
}

fn find_start_pos(grid: &Matrix<char>, c: char) -> (usize, usize) {
    (0..grid.rows)
        .flat_map(|i| (0..grid.columns).map(move |j| (i, j)))
        .find(|&(i, j)| grid[(i, j)] == c)
        .unwrap()
}

fn print_grid_seen(grid: Matrix<char>, seen: Vec<SeenPosition>) {
    // print the grid but replace all seen position grid characters with O
    for i in 0..grid.rows {
        for j in 0..grid.columns {
            let c = if seen.contains(&SeenPosition {
                pos: (i, j),
                steps: 0,
            }) {
                'O'
            } else {
                grid[(i, j)]
            };
            print!("{}", c);
        }
        println!();
    }
}

// The input is a 2d grid of the starting position S, garden plots ., and rocks #.
// The elf can move up, down, left, or right onto a garden plot.
// We need to find how many garden plots the elf can reach in 64 moves.
// This is not just distance, consider the starting position, which is always reachable in 2, 4, 6, etc. steps.
// Or consider any tiles adjacent to the starting position, which is reachable in 1, 3, 5, etc. steps.

// Now imagine the n x n grid is copied infinitely in all directions.
// Find number of reachable garden plots for a much larger number.
fn num_garden_plots_infinite_grid(input: &str, max_steps: usize) -> u64 {
    // Need to note that input is a square and that the row/col of the starting position is empty.
    // Also the outside edge of the grid is empty as well. Thus we can conclude the following:
    // Say we have some reachable tile in the initial grid.
    // The cost of going to the equivalent tile in the adjacent grid is equal to the side length of the grid.
    // That is, the number of steps for a given tile is linear in the side length of the grid.
    // And so the number of tiles reachable in k steps is quadratic because area squares with side length.
    // Let f(s) be the number of tiles reachable in s steps (ie something like num_garden_plots).
    // If n is the side length of the grid, we can construct this quadratic polynomial
    // by interpolating from f(s), f(s+n), f(s+2n).
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Matrix<char>>();

    let grid_size = grid.rows;
    let length_to_edge = (grid_size - 1) / 2;
    println!(
        "Grid size: {}, Length to edge: {}",
        grid_size, length_to_edge
    );

    // ! hardcoded solution
    let expanded_grid = create_expanded_grid(&grid, 7);

    println!("Expanded grid length: {}", expanded_grid.rows);

    // Define f = reachable_garden_plots
    // Note v = very_large_number factors into k = starting to edge = (grid_size-1)/2 and l = grid_size
    // Since v = k + nl, we can write f(v) = f(k+nl)
    // We can thus find f(v) by interpolating the quadratic from f(k), f(k+l), f(k+2l)
    let val1 = reachable_garden_plots(&expanded_grid, length_to_edge);
    let val2 = reachable_garden_plots(&expanded_grid, length_to_edge + grid_size);
    let val3 = reachable_garden_plots(&expanded_grid, length_to_edge + 2 * grid_size);
    println!("val1: {}, val2: {}, val3: {}", val1, val2, val3);
    // So now interpolate from these three values with Lagrange interpolation to find f(v)

    // Lagrange Interpolation to find coefficients
    let a = val1 as f64 / 2.0 - val2 as f64 + val3 as f64 / 2.0;
    let b = -3.0 * val1 as f64 / 2.0 + 2.0 * val2 as f64 - val3 as f64 / 2.0;
    let c = val1 as f64;
    println!("a: {}, b: {}, c: {}", a, b, c);

    let target_step = (max_steps - length_to_edge) / grid_size;
    (a * target_step.pow(2) as f64 + b * target_step as f64 + c) as u64
}

fn create_expanded_grid(original_grid: &Matrix<char>, copies: usize) -> Matrix<char> {
    let rows = original_grid.rows * copies;
    let cols = original_grid.columns * copies;
    let mut expanded_grid = vec![vec!['.'; cols]; rows];

    for i in 0..copies {
        for j in 0..copies {
            for r in 0..original_grid.rows {
                for c in 0..original_grid.columns {
                    let char_to_copy = if i == copies / 2 && j == copies / 2 {
                        original_grid[(r, c)]
                    } else if original_grid[(r, c)] == 'S' {
                        '.'
                    } else {
                        original_grid[(r, c)]
                    };
                    expanded_grid[i * original_grid.rows + r][j * original_grid.columns + c] =
                        char_to_copy;
                }
            }
        }
    }

    // Place 'S' at the center of the entire expanded grid
    let center = rows / 2;
    expanded_grid[center][center] = 'S';

    Matrix::from_rows(expanded_grid).unwrap()
}

fn reachable_garden_plots(grid: &Matrix<char>, max_steps: usize) -> u64 {
    let start_pos = find_start_pos(&grid, 'S');

    // bfs_reach guarantees that we will only visit each tile once
    let seen = bfs_reach(
        SeenPosition {
            pos: start_pos,
            steps: 0,
        },
        |&pos| successors(&grid, pos, max_steps), // This will be the same but using modulo
    )
    .collect_vec();

    // have to filter out equal position SeenPositions (ie steps might be different but .pos is same)
    // since we defined equality for SeenPosition to only care about .pos, we can use unique()
    let seen = seen.into_iter().unique().collect_vec();

    // remove the starting position from seen if max_steps is odd
    let seen = if max_steps % 2 == 1 {
        seen.into_iter()
            .filter(|&pos| pos.pos != start_pos)
            .collect_vec()
    } else {
        seen
    };

    // print_grid_seen(grid.clone(), seen.clone());

    seen.len() as u64
}

#[derive(Clone, Copy, Debug)]
struct SeenPosition {
    pos: (usize, usize),
    steps: usize,
}

impl Hash for SeenPosition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

// two seen positions are equal if their pos is equal, don't care about steps
impl PartialEq for SeenPosition {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for SeenPosition {}

fn successors(grid: &Matrix<char>, seen_pos: SeenPosition, max_steps: usize) -> Vec<SeenPosition> {
    let mut next_positions = Vec::new();
    if &seen_pos.steps >= &max_steps {
        return next_positions;
    }

    let (i, j) = seen_pos.pos;

    if max_steps % 2 == 1 && seen_pos.steps == 0 {
        // At the first step, if max_steps is odd, we can only move to the adjacent tiles,
        // then we can do the normal thing of moving two steps away
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (di, dj) in directions.iter() {
            let new_i = i as isize + di;
            let new_j = j as isize + dj;
            let new_steps = seen_pos.steps + 1;

            if new_i >= 0
                && new_i < grid.rows as isize
                && new_j >= 0
                && new_j < grid.columns as isize
            {
                let new_pos = SeenPosition {
                    pos: (new_i as usize, new_j as usize),
                    steps: new_steps,
                };

                // Check if the position is a garden plot and is reachable in one step
                if grid[new_pos.pos] == '.' {
                    // println!("Found new position: {:?}", new_pos);
                    next_positions.push(new_pos);
                }
            }
        }

        return next_positions;
    }

    let directions = [
        (-2, 0),
        (-1, -1),
        (0, -2),
        (1, -1),
        (2, 0),
        (1, 1),
        (0, 2),
        (-1, 1),
    ];

    for (di, dj) in directions.iter() {
        let new_i = i as isize + di;
        let new_j = j as isize + dj;
        let new_steps = seen_pos.steps + 2;

        if new_i >= 0 && new_i < grid.rows as isize && new_j >= 0 && new_j < grid.columns as isize {
            let new_pos = SeenPosition {
                pos: (new_i as usize, new_j as usize),
                steps: new_steps,
            };

            // Check if the position is a garden plot and is reachable in two steps
            if grid[new_pos.pos] == '.' && is_reachable(grid, (i, j), (*di, *dj)) {
                // println!("Found new position: {:?}", new_pos);
                next_positions.push(new_pos);
            }
        }
    }

    next_positions
}

fn is_reachable(grid: &Matrix<char>, (i, j): (usize, usize), (di, dj): (isize, isize)) -> bool {
    // Check for cardinal directions
    if di.abs() == 2 || dj.abs() == 2 {
        let mid_i = i as isize + di / 2;
        let mid_j = j as isize + dj / 2;
        return grid[(mid_i as usize, mid_j as usize)] != '#';
    }

    // Check for diagonal directions
    if di.abs() == 1 && dj.abs() == 1 {
        return grid[(i, (j as isize + dj) as usize)] != '#'
            || grid[((i as isize + di) as usize, j)] != '#';
    }

    true
}
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

    fn test_input_2() -> &'static str {
        r#".....
.#.#.
..S..
.#.#.
....."#
    }

    #[test]
    fn test_num_garden_plots_example() {
        // let sol1 = num_garden_plots(test_input(), 2);
        // assert_eq!(sol1, 4);
        let sol1 = num_garden_plots(test_input(), 3);
        assert_eq!(sol1, 6);
        // let sol1 = num_garden_plots(test_input(), 6);
        // assert_eq!(sol1, 16);
    }

    #[test]
    fn test_expanded_grid() {
        let input = test_input_2()
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Matrix<char>>();
        let expanded_grid = create_expanded_grid(&input, 5);
        println!("Expanded grid length: {}", expanded_grid.rows);
        print_grid_seen(expanded_grid, vec![]);
    }

    #[test]
    fn test_num_garden_plots_infinite_grid() {
        let sol1 = num_garden_plots_infinite_grid(test_input(), 2);
        let sol1 = num_garden_plots_infinite_grid(test_input(), 6);
        assert_eq!(sol1, 16);
        let sol1 = num_garden_plots_infinite_grid(test_input(), 10);
        assert_eq!(sol1, 50);
        let sol1 = num_garden_plots_infinite_grid(test_input(), 50);
        assert_eq!(sol1, 1594);
        // let sol1 = num_garden_plots_infinite_grid(test_input(), 100);
        // assert_eq!(sol1, 6536);
        // let sol1 = num_garden_plots_infinite_grid(test_input(), 500);
        // assert_eq!(sol1, 167004);
        // let sol1 = num_garden_plots_infinite_grid(test_input(), 1000);
        // assert_eq!(sol1, 668697);
        // let sol1 = num_garden_plots_infinite_grid(test_input(), 5000);
        // assert_eq!(sol1, 16733044);
    }
}
