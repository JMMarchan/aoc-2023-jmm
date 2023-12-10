use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day04.txt").expect("Day 4 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    let sol1 = scratchcard_total_points(&lines);
    let sol2 = total_scratchcards(&lines);

    (Solution::from(sol1), Solution::from(sol2))
}

fn scratchcard_total_points(input: &[&str]) -> u32 {
    // Each line has two list of numbers separated by a pipe.
    // The first list is the winning numbers.
    // The second list is the player's numbers.
    // The player gets points for each number in the player's list that is also in the winning list equal to 2^(matches-1).
    // The player gets 0 points if there are no matches, 1 point if there is 1 match, 2 points if there are 2 matches, 4 points if there are 3 matches, etc.
    // The player's total points is the sum of the points for each line.
    let mut total_points = 0;
    for line in input {
        let matches = count_matches(line);
        total_points += if matches == 0 {
            0
        } else {
            2u32.pow(matches - 1)
        };
    }
    total_points
}

fn count_matches(line: &str) -> u32 {
    let mut parts = line.split(":");
    parts.next();
    let numbers_part = parts.next().unwrap();
    let mut numbers_parts = numbers_part.split("|");
    let winning_numbers = numbers_parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|number| number.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let player_numbers = numbers_parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|number| number.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let matches: u32 = player_numbers
        .iter()
        .filter(|&player_number| winning_numbers.contains(player_number))
        .count() as u32;

    matches
}

// There are no such things as points. Instead, scratchcards cause you to win more scratchcards.
// Specifically, you win copies of the scratchcards below the winning card equal to the number of its winning numbers.
fn total_scratchcards(input: &[&str]) -> u32 {
    let cards: Vec<_> = input
        .iter()
        .map(|line| count_matches(line))
        .collect::<Vec<u32>>();
    let mut queue: std::collections::VecDeque<_> = cards.iter().enumerate().collect();
    let mut total_cards = cards.len() as u32;

    while let Some((index, &matches)) = queue.pop_front() {
        // println!("Current card has {} matches", matches);
        for i in 0..matches {
            let next_index = index + i as usize + 1;
            if let Some(next_matches) = cards.get(next_index) {
                // println!(
                //     "Adding a copy of card {} with {} matches",
                //     next_index + 1,
                //     next_matches
                // );
                queue.push_back((next_index, &next_matches));
                total_cards += 1;
            }
        }
        // println!("Total cards: {}", total_cards);
    }

    total_cards
}
