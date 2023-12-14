use crate::{Solution, SolutionPair};
use hashbrown::HashMap;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day14.txt").expect("Day 14 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    // print height and width of grid
    let height = lines.len();
    let width = lines[0].len();
    println!("Height: {}, Width: {}", height, width);
    let sol1: u64 = total_load(&lines);
    let sol2: u64 = total_load_cycles(&lines, 1_000_000_000);

    (Solution::from(sol1), Solution::from(sol2))
}

// Input is a 2d grid of round rocks (O), cube rocks (#), and spaces (.)
// You can tilt the grid in four directions (up, down, left, right)
// When you tilt the grid, the rounded rocks roll in that direction until they hit a edge of the grid or another rock.
// The load of a rock is the number of rows including itself to the south edge of the grid
// So a rock on the bottom row has a load of 1, a rock on the second to last row has a load of 2, etc.
// Tilt the platform so that all the rounded rocks roll north.
// Find the sum of the loads of all the rounded rocks.
fn total_load(input: &[&str]) -> u64 {
    let mut chars: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    rotate_ccw_and_roll_left(&mut chars);

    let mut total_load = 0;
    // Now the load of each row is the number of rows to the east edge of the grid including itself since we rotated ccw
    // This is just the length of the row - the index of each O
    for row in chars.iter() {
        for (i, c) in row.iter().enumerate() {
            if *c == 'O' {
                total_load += row.len() - i;
            }
        }
    }

    total_load as u64
}

// This function calculates a hash for the grid
fn hash_grid(chars: &Vec<Vec<char>>) -> u64 {
    // let mut hash = 0;
    // for (i, row) in chars.iter().enumerate() {
    //     for (j, &c) in row.iter().enumerate() {
    //         if c == 'O' {
    //             hash ^= 1 << (i * chars.len() + j);
    //         }
    //     }
    // }
    // hash
    // ! THIS WAS WRONG and lead to my main time sink, didn't even realize it for a while, 100x100 grid, so 10000 bits, but u64 is only 64 bits, so it was truncating the hash, resulting in overflows
    let mut hash: u64 = 0;
    for (i, row) in chars.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 'O' {
                // Using a hash function that avoids overflow
                hash =
                    hash.wrapping_add(((i * chars[0].len() + j) as u64).wrapping_mul(2654435761));
            }
        }
    }
    hash
}

// Exactly as expected, we have to do multiple cycles of rotating
// unfortunately, it's 1000000000 cycles, so maybe brute force won't work, but roll north only took 1ms, so that would be 1000 seconds = 16 minutes. Not great, but not terrible.
// also unfortunately, the order is North, West, South, East, so i have the opposite order of what I need
// Apparently, there is a cycle here. See if there is a grid of rocks that we've seen after n cycles, then modulo the number of cycles by the length of the cycle
fn total_load_cycles(input: &[&str], total_cycles: u64) -> u64 {
    let mut chars: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    // let's hash the grid and see if we've seen it before.
    // If so, find 1000000000 % cycle_length and do that many cycles
    // then calculate total load

    let mut seen = HashMap::new();
    let mut first_occurrence = 0;

    let mut cycle_length = None;
    for i in 0..total_cycles {
        let hash = hash_grid(&chars);
        if let Some(&occurence) = seen.get(&hash) {
            first_occurrence = occurence;
            cycle_length = Some(i - first_occurrence);
            println!("Cycle length: {}", cycle_length.unwrap());
            println!(
                "First occurence index: {}, Current index: {}",
                first_occurrence, i
            );
            break;
        }
        seen.insert(hash, i);
        roll_north(&mut chars);
        roll_west(&mut chars);
        roll_south(&mut chars);
        roll_east(&mut chars);
    }

    // First occurrence: 102, Current: 112 (10 cycles)
    // So we see the same grid after 10 steps, at 122, 132, 142, etc.
    // We want the equivalent to 1000000000 steps

    let cycle_length = cycle_length.unwrap_or(total_cycles) as u64;

    // Calculate the total number of complete cycles within the remaining steps
    let remaining_steps = total_cycles - first_occurrence;

    // Calculate the remaining cycles after the last complete cycle
    let remaining_cycles = remaining_steps % cycle_length;

    let final_index = first_occurrence + remaining_cycles;
    println!("Final index: {}", final_index);
    // So it should be the grid at index 110

    // Perform the transformations for the remaining cycles
    for _ in 0..remaining_cycles {
        roll_north(&mut chars);
        roll_west(&mut chars);
        roll_south(&mut chars);
        roll_east(&mut chars);
    }

    calculate_total_load(&mut chars)
}

fn calculate_total_load(chars: &mut Vec<Vec<char>>) -> u64 {
    rotate_grid_ccw(chars);

    let mut total_load = 0;
    for row in chars.iter() {
        for (i, c) in row.iter().enumerate() {
            if *c == 'O' {
                total_load += row.len() - i;
            }
        }
    }

    total_load as u64
}

fn roll_north(chars: &mut Vec<Vec<char>>) {
    // rotate grid ccw then roll left then put grid back.
    rotate_grid_ccw(chars);
    roll_west(chars);
    rotate_grid_cw(chars);
}

fn roll_south(chars: &mut Vec<Vec<char>>) {
    // rotate grid cw then roll left then put grid back.
    rotate_grid_cw(chars);
    roll_west(chars);
    rotate_grid_ccw(chars);
}

fn roll_east(chars: &mut Vec<Vec<char>>) {
    // rotate grid ccw twice then roll left then put grid back.
    rotate_grid_ccw(chars);
    rotate_grid_ccw(chars);
    roll_west(chars);
    rotate_grid_cw(chars);
    rotate_grid_cw(chars);
}

// this will certainly have to be generalized to handle all four directions but I'm starting with north

fn rotate_ccw_and_roll_left(chars: &mut Vec<Vec<char>>) {
    // rotate grid ccw then roll left
    rotate_grid_ccw(chars);
    roll_west(chars);
}

// It is a square (100x100 for input), so rotation is easy
fn rotate_grid_ccw(chars: &mut Vec<Vec<char>>) {
    let n = chars.len();
    for i in 0..n / 2 {
        for j in i..n - i - 1 {
            // Store current cell in temp variable
            let temp = chars[i][j];
            // Move values from right to top
            chars[i][j] = chars[j][n - 1 - i];
            // Move values from bottom to right
            chars[j][n - 1 - i] = chars[n - 1 - i][n - 1 - j];
            // Move values from left to bottom
            chars[n - 1 - i][n - 1 - j] = chars[n - 1 - j][i];
            // Assign temp to left
            chars[n - 1 - j][i] = temp;
        }
    }
} // fn rotate_grid_ccw

fn rotate_grid_cw(chars: &mut Vec<Vec<char>>) {
    let n = chars.len();
    for i in 0..n / 2 {
        for j in i..n - i - 1 {
            // Store current cell in temp variable
            let temp = chars[i][j];
            // Move values from left to top
            chars[i][j] = chars[n - 1 - j][i];
            // Move values from bottom to left
            chars[n - 1 - j][i] = chars[n - 1 - i][n - 1 - j];
            // Move values from right to bottom
            chars[n - 1 - i][n - 1 - j] = chars[j][n - 1 - i];
            // Assign temp to right
            chars[j][n - 1 - i] = temp;
        }
    }
}

fn roll_west(chars: &mut Vec<Vec<char>>) {
    for row in chars.iter_mut() {
        // Split the row by '#', process each group, and collect the results
        let processed: Vec<Vec<char>> = row
            .split(|&c| c == '#')
            .map(|group| {
                let mut result: Vec<char> = group.iter().filter(|&&c| c == 'O').cloned().collect();
                result.extend(vec!['.'; group.len() - result.len()]);
                result
            })
            .collect();

        // Flatten the processed groups and interleave '#'s
        *row = processed
            .into_iter()
            .flat_map(|mut group| {
                group.push('#');
                group
            })
            .collect();

        // Remove the extra '#' added at the end
        row.pop();
    }
}

fn _function2(input: &[&str]) -> u64 {
    0
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
    fn test_multiple_cycles() {
        let input = test_input();
        let expected = test_expected_after_1_cycle();
        let mut chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        roll_north(&mut chars);
        roll_west(&mut chars);
        roll_south(&mut chars);
        roll_east(&mut chars);
        assert_eq!(
            chars,
            expected
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        );

        let expected = test_expected_after_2_cycles();
        roll_north(&mut chars);
        roll_west(&mut chars);
        roll_south(&mut chars);
        roll_east(&mut chars);
        assert_eq!(
            chars,
            expected
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        );
    }

    #[test]
    fn test_roll_left() {
        let input = ".O.#.OO.#.";
        let expected = "O..#OO..#.";
        let mut chars: Vec<Vec<char>> = vec![input.chars().collect()];
        roll_west(&mut chars);
        assert_eq!(chars[0], expected.chars().collect::<Vec<char>>());
    }

    #[test]
    fn test_total_load() {
        let input = test_input();
        let input: Vec<&str> = input.lines().collect();
        assert_eq!(total_load(&input), 136);
    }

    #[test]
    fn test_rotate_grid() {
        let input = r#".#
##"#;
        let mut chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        rotate_grid_ccw(&mut chars);
        let expected = r#"##
.#"#;
        let expected: Vec<Vec<char>> = expected
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        assert_eq!(chars, expected);

        rotate_grid_cw(&mut chars);

        let expected = r#".#
##"#;
        let expected: Vec<Vec<char>> = expected
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        assert_eq!(chars, expected);
    }
}
