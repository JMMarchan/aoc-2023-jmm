use crate::{Solution, SolutionPair};
use grid::Grid;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use std::cmp::{max, min};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day18.txt").expect("Day 18 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    let sol1 = lava_dig_plan(&lines);
    let sol2 = lava_dig_plan_alternate(&lines);

    (Solution::from(sol1), Solution::from(sol2))
}

// The input is a line separated list of strings. Each string is a direction, a number of steps, and a color in RGB hex format.
// The digger starts in a 1 meter cube hole in the ground, then they dig in the direction with the number of steps.
// Each trench is also listed with the color that the edge of the trench should be painted.
// The instructions create a loop of trenches that the digger will dig.
// After these trenches are dug, the digger then digs out the interior of the loop.
// Find the number of cubic meters of dirt that the digger will dig.
// TODO: just use the same Pick's theorem algorithm as part 2
fn lava_dig_plan(input: &[&str]) -> usize {
    let mut trenches: HashMap<(isize, isize), Direction> = HashMap::new();
    let mut digger = (0, 0);
    let mut turning_number = 0;
    let mut last = None;
    let loop_trenches = input.iter().fold(0, |acc, line| {
        let mut parts = line.split_whitespace();
        let dir = Direction::from_str(parts.next().unwrap());
        turning_number += update_turning_number(last, dir);
        last = Some(dir);
        let steps = parts.next().unwrap().parse::<usize>().unwrap();
        let (di, dj) = dir.to_delta();
        let mut num_trenches = acc;
        for _ in 0..steps {
            digger.0 += di;
            digger.1 += dj;
            trenches.insert(digger, dir);
            num_trenches += 1;
        }
        num_trenches
    });

    // println!(
    //     "Turning number: {}, Loop orientation: {}",
    //     turning_number,
    //     if turning_number > 0 {
    //         "clockwise"
    //     } else {
    //         "counterclockwise"
    //     }
    // );
    // Find a safe starting point for the flood fill
    let (start_i, start_j) = find_start_point(&trenches, turning_number);

    // Perform flood fill to find the number of interior trenches
    let interior_trenches = flood_fill(&trenches, start_i, start_j);

    // println!("Loop trenches: {}", loop_trenches);
    // println!("Interior trenches: {}", interior_trenches);

    loop_trenches + interior_trenches
}

fn find_start_point(
    trenches: &HashMap<(isize, isize), Direction>,
    turning_number: isize,
) -> (isize, isize) {
    for (&(i, j), &dir) in trenches.iter() {
        match dir {
            Direction::Right if turning_number > 0 => return (i + 1, j),
            Direction::Down if turning_number > 0 => return (i, j - 1),
            Direction::Left if turning_number > 0 => return (i - 1, j),
            Direction::Up if turning_number > 0 => return (i, j + 1),
            _ => (),
        }
    }
    unreachable!("Should have found a starting point")
}

fn flood_fill(
    trenches: &HashMap<(isize, isize), Direction>,
    start_i: isize,
    start_j: isize,
) -> usize {
    let mut stack: Vec<(isize, isize)> = vec![(start_i, start_j)];
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut count = 0;

    while let Some((i, j)) = stack.pop() {
        if !visited.insert((i, j)) {
            continue;
        }
        if !trenches.contains_key(&(i, j)) {
            count += 1;
            for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                let ni = i + di;
                let nj = j + dj;
                if !trenches.contains_key(&(ni, nj)) {
                    stack.push((ni, nj));
                }
            }
        }
    }
    count
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }

    fn from_num(n: u32) -> Self {
        match n {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => panic!("Invalid direction"),
        }
    }

    fn to_delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

fn update_turning_number(last_dir: Option<Direction>, current_dir: Direction) -> isize {
    if let Some(last_dir) = last_dir {
        match (last_dir, current_dir) {
            (Direction::Up, Direction::Right)
            | (Direction::Right, Direction::Down)
            | (Direction::Down, Direction::Left)
            | (Direction::Left, Direction::Up) => 1,
            (Direction::Right, Direction::Up)
            | (Direction::Down, Direction::Right)
            | (Direction::Left, Direction::Down)
            | (Direction::Up, Direction::Left) => -1,
            _ => 0,
        }
    } else {
        0
    }
}

// This time, the number of steps is encoded in the color of the trench.
// Each hexidecimal code is six digits long. The first five are a five-digit hexadecimal number for the number of steps.
// The last hexidecimal digit is the direction where: 0 = up, 1 = down, 2 = left, 3 = right.
// Examples:
// #70c710 = R 461937
// #0dc571 = D 56407
// #5713f0 = R 356671
// Find the number of cubic meters of dirt that the digger will dig.
fn lava_dig_plan_alternate(input: &[&str]) -> u64 {
    // Now original algorithm and flood fill is unfeasible with the number of trenches.
    // Shoelace formula? Only store vertices? Pick's theorem
    let mut vertices: Vec<(i64, i64)> = Vec::new();
    let mut position: (i64, i64) = (0, 0);

    let mut boundary_points = 0;
    // Collect directions and steps
    input.iter().for_each(|&line| {
        // We don't care about the first two parts anymore, just the color
        let start = line.find('(').unwrap() + 1;
        let end = line.find(')').unwrap();
        let color = &line[start..end];
        let steps = i64::from_str_radix(&color[1..6], 16).unwrap(); // Scale steps
        let dir = Direction::from_num(color.chars().last().unwrap().to_digit(16).unwrap());

        // Update position based on current direction and steps
        let (di, dj) = dir.to_delta();
        position.0 += (di as i64) * steps;
        position.1 += (dj as i64) * steps;
        boundary_points += steps;
        vertices.push(position);
    });

    let area = shoelace_formula(&vertices);

    println!("Boundary points: {}, Area: {}", boundary_points, area);
    // Pick's theorem states that area = interior_points + boundary_points / 2 - 1
    // Thus interior points = area - boundary_points / 2 + 1
    // total_points = interior_points + boundary_points = area - boundary_points / 2 + 1 + boundary_points
    let boundary_points = boundary_points as u64;
    area - boundary_points / 2 + 1 + boundary_points
}

fn shoelace_formula(vertices: &[(i64, i64)]) -> u64 {
    let n = vertices.len();
    let mut area = 0;

    for i in 0..n {
        let (x1, y1) = vertices[i];
        let (x2, y2) = if i == n - 1 {
            vertices[0]
        } else {
            vertices[i + 1]
        };

        area += x1 * y2 - x2 * y1;
    }

    (area.abs() as u64) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"#
    }

    // This digs out the following (where # is trench)
    // #######
    // #.....#
    // ###...#
    // ..#...#
    // ..#...#
    // ###.###
    // #...#..
    // ##..###
    // .#....#
    // .######
    // Then the interior is dug out, so we count the number of #s, which would be 62.
    // #######
    // #######
    // #######
    // ..#####
    // ..#####
    // #######
    // #####..
    // #######
    // .######
    // .######

    #[test]
    fn test_dig_plan_lava_sample() {
        let input = test_input().lines().collect::<Vec<&str>>();
        assert_eq!(lava_dig_plan(&input), 62);
    }

    #[test]
    fn test_dig_plan_lava_alternate_sample() {
        let input = test_input().lines().collect::<Vec<&str>>();
        assert_eq!(lava_dig_plan_alternate(&input), 952408144115);
    }

    #[test]
    fn test_dig_plan_lava_alternate_simple() {
        let input = vec![
            "R 2 (#000020)",
            "D 2 (#000021)",
            "L 2 (#000022)",
            "U 2 (#000023)",
        ];
        assert_eq!(lava_dig_plan_alternate(&input), 9);
    }

    #[test]
    fn test_shoelace_formula() {
        let vertices = vec![(0, 0), (4, 0), (4, 4), (0, 4)];
        assert_eq!(shoelace_formula(&vertices), 16);
    }
}
