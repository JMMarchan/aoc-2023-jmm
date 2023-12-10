use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day02.txt").expect("Day 2 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    let sol1 = sum_of_valid_games(&lines);
    let sol2 = sum_of_games_power(&lines);

    (Solution::from(sol1), Solution::from(sol2))
}

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

// Each game, after a colon, is a semi-colon separated list of plays, where a play is a  comma-separated lists of colors.
// Each color is a number followed by a space and a color name.
// The number is the number of cubes of that color.
// The color name is one of "red", "green", or "blue".
// A valid play has at most 12 red cubes, 13 green cubes, and 14 blue cubes.
// A valid game only has valid plays.
// The "id" of a game is its number.
// Get the sum of the ids of all valid games.
fn sum_of_valid_games(input: &[&str]) -> u32 {
    input
        .iter()
        .filter_map(|line| {
            let mut parts = line.split(": ");
            let id = parts
                .next()?
                .trim_start_matches("Game ")
                .parse::<u32>()
                .ok()?;
            let plays = parts.next()?;
            if plays.split("; ").all(|play| is_play_valid(play)) {
                Some(id)
            } else {
                None
            }
        })
        .sum()
}

// fn is_play_valid(play: &str) -> bool {
//     let mut red_cubes = 0;
//     let mut green_cubes = 0;
//     let mut blue_cubes = 0;

//     for color in play.split(", ") {
//         let mut parts = color.split(" ");
//         let count = parts
//             .next()
//             .and_then(|c| c.parse::<u32>().ok())
//             .unwrap_or(0);
//         match parts.next() {
//             Some("red") if count <= MAX_RED_CUBES => red_cubes += count, // ? what
//             Some("green") if count <= MAX_GREEN_CUBES => green_cubes += count,
//             Some("blue") if count <= MAX_BLUE_CUBES => blue_cubes += count,
//             _ => return false,
//         }
//     }

//     true
// }

fn is_play_valid(play: &str) -> bool {
    let mut red_cubes = 0;
    let mut green_cubes = 0;
    let mut blue_cubes = 0;

    for color in play.split(", ") {
        let mut parts = color.split(" ");
        let count = parts
            .next()
            .and_then(|c| c.parse::<u32>().ok())
            .unwrap_or(0);
        match parts.next() {
            Some("red") if count <= MAX_RED_CUBES => red_cubes += count,
            Some("green") if count <= MAX_GREEN_CUBES => green_cubes += count,
            Some("blue") if count <= MAX_BLUE_CUBES => blue_cubes += count,
            _ => return false,
        }
    }

    red_cubes <= MAX_RED_CUBES && green_cubes <= MAX_GREEN_CUBES && blue_cubes <= MAX_BLUE_CUBES
}

// For each game, find the minimum number of cubes to make a valid game.
// The minimum number of cubes needed is the maximum number of cubes for each color.
// The power of a game is the product of the minimum number of cubes for each game.
// Get the sum of the powers of all games.
fn sum_of_games_power(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| {
            let plays = line.split(": ").nth(1).unwrap_or("");
            let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);

            for play in plays.split("; ") {
                let (mut red_cubes, mut green_cubes, mut blue_cubes) = (0, 0, 0);
                for color in play.split(", ") {
                    let mut parts = color.split(" ");
                    let count = parts
                        .next()
                        .and_then(|c| c.parse::<u32>().ok())
                        .unwrap_or(0);
                    match parts.next() {
                        Some("red") => red_cubes = count.max(red_cubes),
                        Some("green") => green_cubes = count.max(green_cubes),
                        Some("blue") => blue_cubes = count.max(blue_cubes),
                        _ => (),
                    }
                }
                max_red = max_red.max(red_cubes);
                max_green = max_green.max(green_cubes);
                max_blue = max_blue.max(blue_cubes);
            }

            max_red * max_green * max_blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day02_test_input() -> &'static str {
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    }

    #[test]
    fn test_sum_of_valid_games() {
        let input = day02_test_input();
        // split into lines
        let input: Vec<&str> = input.lines().collect();
        assert_eq!(sum_of_valid_games(&input), 8);
    }

    #[test]
    fn test_sum_of_games_power() {
        let input = day02_test_input();
        // split into lines
        let input: Vec<&str> = input.lines().collect();
        assert_eq!(sum_of_games_power(&input), 2286);
    }
}
