use crate::{Solution, SolutionPair};
use hashbrown::HashMap;
use rayon::prelude::*;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day14.txt").expect("Day 14 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    // print height and width of grid
    // let height = lines.len();
    // let width = lines[0].len();
    // println!("Height: {}, Width: {}", height, width);
    let sol1: u64 = total_load_col(&lines);
    let sol2: u64 = total_load_cycles(&lines, 1_000_000_000);

    (Solution::from(sol1), Solution::from(sol2))
}

// The solution has been refactored in general to use a vector of columns instead of rows
fn total_load_cycles(input: &[&str], total_cycles: u64) -> u64 {
    let mut chars_col: Vec<Vec<char>> = rows_to_cols(input);

    // let's hash the grid and see if we've seen it before.
    // If so, find 1000000000 % cycle_length and do that many cycles
    // then calculate total load

    let mut seen = HashMap::new();
    let mut first_occurrence = 0;

    let mut cycle_length = None;
    for i in 0..total_cycles {
        let hash = hash_grid_col(&chars_col);
        if let Some(&occurence) = seen.get(&hash) {
            // This is basically Floyd's cycle detection algorithm
            first_occurrence = occurence;
            cycle_length = Some(i - first_occurrence);
            // println!("Cycle length: {}", cycle_length.unwrap());
            // println!(
            //     "First occurence index: {}, Current index: {}",
            //     first_occurrence, i
            // );
            break;
        }
        seen.insert(hash, i);
        roll_cycle_col(&mut chars_col);
    }

    // First occurrence: 102, Current: 112 (10 cycles)
    // So we see the same grid after 10 steps, at 122, 132, 142, etc.
    // We want the equivalent to 1000000000 steps

    let cycle_length = cycle_length.unwrap_or(total_cycles) as u64;

    // Calculate the total number of complete cycles within the remaining steps
    let remaining_steps = total_cycles - first_occurrence;

    // Calculate the remaining cycles after the last complete cycle
    let remaining_cycles = remaining_steps % cycle_length;

    // let final_index = first_occurrence + remaining_cycles;
    // println!("Final index: {}", final_index);
    // So it should be the grid at index 118

    // Perform the transformations for the remaining cycles
    for _ in 0..remaining_cycles {
        roll_cycle_col(&mut chars_col);
    }

    calculate_total_load_col(chars_col)
}

fn hash_grid_col(chars_col: &Vec<Vec<char>>) -> u64 {
    // ! THIS WAS WRONG and lead to my main time sink, didn't even realize it for a while, 100x100 grid, so 10000 bits, but u64 is only 64 bits, so it was truncating the hash, resulting in overflows. I'm keeping this here as a reminder to be careful with bit operations
    // ! This is based on my old row hashing function, but it's not quite right, since we're hashing columns now, but I'm still keeping it here.
    // let mut hash = 0;
    // for (i, row) in chars.iter().enumerate() {
    //     for (j, &c) in row.iter().enumerate() {
    //         if c == 'O' {
    //             hash ^= 1 << (i * chars.len() + j);
    //         }
    //     }
    // }
    // hash
    let mut hash: u64 = 0;
    for (i, col) in chars_col.iter().enumerate() {
        for (j, &c) in col.iter().enumerate() {
            if c == 'O' {
                // Using a hash function that avoids overflow
                hash = hash
                    .wrapping_add(((i * chars_col[0].len() + j) as u64).wrapping_mul(2654435761));
            }
        }
    }
    hash
}

// rows_to_cols takes in a slice of strings representing rows and returns a vector of columns
fn rows_to_cols(input: &[&str]) -> Vec<Vec<char>> {
    // Determine the number of columns
    let num_columns = input.first().map_or(0, |line| line.len());

    // Initialize a vector of columns
    let mut chars_col: Vec<Vec<char>> = vec![Vec::new(); num_columns];

    // Fill each column with characters from the input
    for line in input.iter() {
        for (col, char) in line.chars().enumerate() {
            chars_col[col].push(char);
        }
    }

    chars_col
}

// Input is a 2d grid of round rocks (O), cube rocks (#), and spaces (.)
// You can tilt the grid in four directions (up, down, left, right)
// When you tilt the grid, the rounded rocks roll in that direction until they hit a edge of the grid or another rock.
// The load of a rock is the number of rows including itself to the south edge of the grid
// So a rock on the bottom row has a load of 1, a rock on the second to last row has a load of 2, etc.
// Tilt the platform so that all the rounded rocks roll north.
// Find the sum of the loads of all the rounded rocks.
fn total_load_col(input: &[&str]) -> u64 {
    let mut chars_col: Vec<Vec<char>> = rows_to_cols(input);
    roll_north_col(&mut chars_col);
    calculate_total_load_col(chars_col)
}

fn calculate_total_load_col(chars_col: Vec<Vec<char>>) -> u64 {
    let mut total_load = 0;
    for col in chars_col.iter() {
        for (i, c) in col.iter().enumerate() {
            if *c == 'O' {
                total_load += col.len() - i;
            }
        }
    }

    total_load as u64
}

// Perform a north, west, south, east roll cycle on the grid
fn roll_cycle_col(chars_col: &mut Vec<Vec<char>>) {
    for _ in 0..4 {
        roll_north_col(chars_col);
        rotate_grid_cw_col(chars_col);
    }
}

// For each column, move the rocks 'O' up to the next # or the top of the grid
fn roll_north_col(chars_col: &mut Vec<Vec<char>>) {
    chars_col.par_iter_mut().for_each(|col| {
        // Split the column by '#', process each group, and collect the results
        let processed: Vec<Vec<char>> = col
            .split(|&c| c == '#')
            .map(|group| {
                let mut result: Vec<char> = group.iter().filter(|&&c| c == 'O').cloned().collect();
                result.extend(vec!['.'; group.len() - result.len()]);
                result
            })
            .collect();

        // Flatten the processed groups and interleave '#'s
        *col = processed
            .into_iter()
            .flat_map(|mut group| {
                group.push('#');
                group
            })
            .collect();

        // Remove the extra '#' added at the end
        col.pop();
    });
}

// This takes in a vector of columns, rotating the characters in the grid clockwise
fn rotate_grid_cw_col(chars: &mut Vec<Vec<char>>) {
    let n = chars.len();
    let m = chars[0].len();
    let mut new_grid = vec![vec!['.'; n]; m];

    for i in 0..n {
        for j in 0..m {
            new_grid[m - 1 - j][i] = chars[i][j];
        }
    }

    *chars = new_grid;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#
    }

    fn test_expected_after_1_cycle() -> &'static str {
        r#".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."#
    }

    fn test_expected_after_2_cycles() -> &'static str {
        r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"#
    }

    #[test]
    fn test_rotate_grid_cw_col() {
        let input = r#"#.
O."#;
        let mut chars: Vec<Vec<char>> = rows_to_cols(&input.lines().collect::<Vec<&str>>());
        let expected = r#"O#
.."#;
        let expected: Vec<Vec<char>> = rows_to_cols(&expected.lines().collect::<Vec<&str>>());
        rotate_grid_cw_col(&mut chars);

        assert_eq!(chars, expected);
    }

    #[test]
    fn test_roll_north_col() {
        let input = r#"#.
O.
.O
OO"#;
        let expected = r#"#O
OO
O.
.."#;
        let mut chars: Vec<Vec<char>> = rows_to_cols(&input.lines().collect::<Vec<&str>>());
        let expected = rows_to_cols(&expected.lines().collect::<Vec<&str>>());
        roll_north_col(&mut chars);
        assert_eq!(chars, expected);
    }

    #[test]
    fn test_multiple_cycles_col() {
        let input = test_input();
        let expected = test_expected_after_1_cycle();
        let mut chars: Vec<Vec<char>> = rows_to_cols(&input.lines().collect::<Vec<&str>>());
        let expected_cols = rows_to_cols(&expected.lines().collect::<Vec<&str>>());
        roll_cycle_col(&mut chars);
        assert_eq!(chars, expected_cols);

        let expected = test_expected_after_2_cycles();
        let expected_cols = rows_to_cols(&expected.lines().collect::<Vec<&str>>());
        roll_cycle_col(&mut chars);
        assert_eq!(chars, expected_cols);
    }
}
