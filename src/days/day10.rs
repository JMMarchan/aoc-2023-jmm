use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;
use std::rc::Rc;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // The starting position is a north-west bend in the puzzle input
    let input = read_to_string("input/day10.txt").expect("Day 10 input file should be present.");
    let input: Vec<&str> = input.lines().collect();
    let sol1: u64 = farthest_distance_in_loop(&input, TileType::NorthWestBend); // TODO: This should be more general instead of a hardcoded start type
    let sol2: u64 = tiles_enclosed_by_loop(&input, TileType::NorthWestBend);

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Tile {
    tile_type: TileType,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum TileType {
    VerticalPipe,
    HorizontalPipe,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
}

impl TileType {
    fn from_char(c: char, start_type: TileType) -> Self {
        match c {
            '|' => Self::VerticalPipe,
            '-' => Self::HorizontalPipe,
            'L' => Self::NorthEastBend,
            'J' => Self::NorthWestBend,
            '7' => Self::SouthWestBend,
            'F' => Self::SouthEastBend,
            '.' => Self::Ground,
            'S' => start_type,
            _ => panic!("Invalid tile type: {}", c),
        }
    }

    fn valid_directions(&self) -> Vec<Direction> {
        match self {
            Self::VerticalPipe => vec![Direction::North, Direction::South],
            Self::HorizontalPipe => vec![Direction::East, Direction::West],
            Self::NorthEastBend => vec![Direction::North, Direction::East],
            Self::NorthWestBend => vec![Direction::North, Direction::West],
            Self::SouthWestBend => vec![Direction::South, Direction::West],
            Self::SouthEastBend => vec![Direction::South, Direction::East],
            Self::Ground => vec![],
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Grid {
    tiles: HashMap<(usize, usize), Tile>,
    grid_height: usize,
    grid_width: usize,
}

impl Grid {
    fn new(tiles: HashMap<(usize, usize), Tile>, grid_height: usize, grid_width: usize) -> Self {
        Self {
            tiles,
            grid_height,
            grid_width,
        }
    }
}

// The input is a list of lines, each of which is a list of characters, creating a 2d grid of tiles.
// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
// The animal starts at S and moves through the pipes, which form a single loop.
// There are tiles outside of the loop which we must ignore.
// Find the distance from S to the farthest tile in the loop (going either way around the loop).
fn farthest_distance_in_loop(input: &[&str], start_type: TileType) -> u64 {
    let grid = parse_grid(input, start_type);
    let start_position = find_start_position(input);
    // println!("start_position: {:?}", start_position);
    // println!("grid: {:?}", grid);
    bfs(&grid, start_position).0
}

fn parse_grid(input: &[&str], start_type: TileType) -> Grid {
    let mut grid = HashMap::new();

    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            let tile_type = TileType::from_char(c, start_type.clone());
            let tile = Tile { tile_type };
            grid.insert((i, j), tile);
        }
    }

    let grid_height = input.len();
    let grid_width = input[0].len();

    Grid::new(grid, grid_height, grid_width)
}

fn find_start_position(input: &[&str]) -> (usize, usize) {
    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c == 'S' {
                return (i, j);
            }
        }
    }

    panic!("No start position found");
}

fn bfs(grid: &Grid, start_position: (usize, usize)) -> (u64, HashSet<(usize, usize)>) {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    queue.push_back((start_position, 0));

    while let Some((position, distance)) = queue.pop_front() {
        // println!("position: {:?}, distance: {}", position, distance);
        if visited.contains_key(&position) {
            continue;
        }
        visited.insert(position, distance);

        let tile = grid.tiles.get(&position).unwrap();

        // Add valid neighbors to the queue
        let neighbors = get_valid_neighbors(position, tile, grid);
        for neighbor in neighbors {
            if !visited.contains_key(&neighbor) {
                queue.push_back((neighbor, distance + 1));
            }
        }
    }

    // create hash set from keys of visited hash map to get the loop tiles

    (
        *visited.values().max().unwrap(),
        visited.keys().cloned().collect(),
    )
}

// Based on the tile type, get the valid neighbors of the tile.
fn get_valid_neighbors(pos: (usize, usize), tile: &Tile, grid: &Grid) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let (i, j) = pos;

    // let (src_dir1, src_dir2) = tile.tile_type.valid_directions().split_at(1);
    // valid_directions returns a list of directions that are valid for the tile type, if it's ground, then there are no valid directions, so we need to handle that case too

    let valid_directions = tile.tile_type.valid_directions();

    // println!("valid_directions: {:?}", valid_directions);

    // North
    if valid_directions.contains(&Direction::North) && i > 0 {
        if let Some(north_tile) = grid.tiles.get(&(i - 1, j)) {
            let north_tile_valid_directions = north_tile.tile_type.valid_directions();
            if north_tile_valid_directions.contains(&Direction::South) {
                neighbors.push((i - 1, j));
            }
        }
    }

    // South
    if valid_directions.contains(&Direction::South) && i < grid.grid_height - 1 {
        if let Some(south_tile) = grid.tiles.get(&(i + 1, j)) {
            let south_tile_valid_directions = south_tile.tile_type.valid_directions();
            if south_tile_valid_directions.contains(&Direction::North) {
                neighbors.push((i + 1, j));
            }
        }
    }

    // East
    if valid_directions.contains(&Direction::East) && j < grid.grid_width - 1 {
        if let Some(east_tile) = grid.tiles.get(&(i, j + 1)) {
            let east_tile_valid_directions = east_tile.tile_type.valid_directions();
            if east_tile_valid_directions.contains(&Direction::West) {
                neighbors.push((i, j + 1));
            }
        }
    }

    // West
    if valid_directions.contains(&Direction::West) && j > 0 {
        if let Some(west_tile) = grid.tiles.get(&(i, j - 1)) {
            let west_tile_valid_directions = west_tile.tile_type.valid_directions();
            if west_tile_valid_directions.contains(&Direction::East) {
                neighbors.push((i, j - 1));
            }
        }
    }
    // println!("neighbors: {:?}", neighbors);

    neighbors
}

// In the second part, we find the number of tiles enclosed by the loop.
fn tiles_enclosed_by_loop(input: &[&str], start_type: TileType) -> u64 {
    let grid = parse_grid(input, start_type);
    let start_position = find_start_position(input);
    let loop_tiles: HashSet<(usize, usize)> = bfs(&grid, start_position).1;
    // JORDAN CURVE THEOREM: A simple closed curve divides the plane into two regions, the inside and the outside.
    // If a simple closed curve crosses a line an odd number of times, then the line is inside the curve. If a simple closed curve crosses a line an even number of times, then the line is outside the curve.
    // We can take advantage of the fact that the loop is a simple closed curve to find the tiles enclosed by the loop

    let mut enclosed_tiles_count = 0;

    // Looping over each row from left to right, we count the number of vertical crossings we've seen
    // A vertical crossing is as loop-tile that is: a VerticalPipe; a NorthWestBend preceeded by zero or more HorizontalPipes and then a SouthEastBend; a SouthWestBend preceeded by zero or more HorizontalPipes and then a NorthEastBend
    // An enclosed tile is a non-loop tile that has an odd number of vertical crossings
    for i in 0..grid.grid_height {
        let mut vertical_crossings = 0;

        for j in 0..grid.grid_width {
            // Check if the tile is a loop tile
            if loop_tiles.contains(&(i, j)) {
                // Check if it's a vertical crossing
                // If it is, then increment the vertical crossings count
                if let Some(tile) = grid.tiles.get(&(i, j)) {
                    // Imagine drawing a line through the top quarter of the tile instead of the half
                    // Then counting NE or NW bends is the same as counting vertical pipes as a vertical crossing.
                    match tile.tile_type {
                        TileType::VerticalPipe
                        | TileType::NorthEastBend
                        | TileType::NorthWestBend => {
                            vertical_crossings += 1;
                        }
                        _ => {}
                    }
                }
            } else if vertical_crossings % 2 != 0 {
                // If it's not a loop tile, and it has odd parity vertical crossing, then it's an enclosed tile
                enclosed_tiles_count += 1;
            }
        }
    }

    enclosed_tiles_count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day10_test_input() -> &'static str {
        "..F7.\n\
.FJ|.\n\
SJ.L7\n\
|F--J\n\
LJ..."
    }

    fn day10_test_input_2() -> &'static str {
        "FF7FSF7F7F7F7F7F---7\n\
L|LJ||||||||||||F--J\n\
FL-7LJLJ||||||LJL-77\n\
F--JF--7||LJLJ7F7FJ-\n\
L---JF-JLJ.||-FJLJJ7\n\
|F|F-JF---7F7-L7L|7|\n\
|FFJF7L7F-JF7|JL---7\n\
7-L-JL7||F7|L7F-7F7|\n\
L.L7LFJ|||||FJL7||LJ\n\
L7JLJL-JLJLJL--JLJ.L\n\
"
    }

    #[test]
    fn test_farthest_distance_in_loop() {
        let input = day10_test_input();
        let input: Vec<&str> = input.lines().collect();
        assert_eq!(
            farthest_distance_in_loop(&input, TileType::SouthEastBend),
            8
        );
    }

    #[test]
    fn test_tiles_enclosed_by_loop() {
        let input1 = day10_test_input();
        let input1: Vec<&str> = input1.lines().collect();
        assert_eq!(tiles_enclosed_by_loop(&input1, TileType::SouthEastBend), 1);
        let input2 = day10_test_input_2();
        let input2: Vec<&str> = input2.lines().collect();
        // assert_eq!(tiles_enclosed_by_loop(&input1, TileType::SouthEastBend), 1);
        assert_eq!(tiles_enclosed_by_loop(&input2, TileType::SouthWestBend), 10);
    }
}
