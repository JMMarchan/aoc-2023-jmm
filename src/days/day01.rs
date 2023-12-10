use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day01.txt").expect("Day 1 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    let sol1 = concatenate_and_sum(&lines);
    let sol2 = parse_concat_sum(&lines);

    (Solution::from(sol1), Solution::from(sol2))
}

// For each line, concatenate the first and last digits of the line, and sum the results.
fn concatenate_and_sum(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| {
            let first_digit = line.chars().find(|c| c.is_digit(10)).unwrap();
            let last_digit = line.chars().rev().find(|c| c.is_digit(10)).unwrap();
            format!("{}{}", first_digit, last_digit)
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

// For each line, concatenate the first and last digits of the line, and sum the results.
// However, the words "one, two, ..., nine" now count as "digits" in addition to the usual digits 0-9
// There may not be real digits.
// A find and replace won't work, consider eighthree, which would become 8hree.
fn parse_concat_sum(input: &[&str]) -> u32 {
    input
        .iter()
        .filter_map(|line| {
            let first_digit = find_digit(line, false).unwrap();
            let last_digit = find_digit(line, true).unwrap();
            // Concatenate the first and last digits, parse it as an integer, or default to 0 if parsing fails.
            Some(
                format!("{}{}", first_digit, last_digit)
                    .parse::<u32>()
                    .ok()
                    .unwrap_or(0),
            )
        })
        .sum()
}

const DIGIT_WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

// Finds either the first or last digit (depending on find_last) in a given line.
fn find_digit(line: &str, find_last: bool) -> Option<u32> {
    let mut first_occurrences = Vec::new();
    let mut last_occurrences = Vec::new();

    // Iterate over each word in DIGIT_WORDS to find their occurrences.
    for &word in DIGIT_WORDS.iter() {
        if let Some(index) = line.find(word) {
            first_occurrences.push((index, word));
        }
        if let Some(index) = line.rfind(word) {
            last_occurrences.push((index, word));
        }
    }

    // Sort the occurrences to find the first and last word-digit.
    first_occurrences.sort_by_key(|&(index, _)| index);
    last_occurrences.sort_by_key(|&(index, _)| index);

    let word_digit = if find_last {
        last_occurrences.last().cloned()
    } else {
        first_occurrences.first().cloned()
    };

    // Find either the first or last usual digit.
    let usual_digit = if find_last {
        line.chars()
            .rev()
            .enumerate()
            .find(|&(_, c)| c.is_digit(10))
            .map(|(i, c)| (line.len() - 1 - i, c))
    } else {
        line.chars().enumerate().find(|(_, c)| c.is_digit(10))
    };

    // Determine which digit comes first/last and convert to a number.
    match (word_digit, usual_digit) {
        // If both word and usual digit are found, select based on their indices and the find_last flag.
        (Some((idx_word, word)), Some((idx_usual, _)))
            if (find_last && idx_word > idx_usual) || (!find_last && idx_word < idx_usual) =>
        {
            DIGIT_WORDS
                .iter()
                .position(|&w| w == word)
                .map(|pos| pos as u32) // Convert word to its corresponding digit.
        }
        // If only a usual digit is found, convert it to a number.
        (_, Some((_, digit))) => Some(digit.to_digit(10).unwrap()),
        // If only a word digit is found, convert it to its corresponding number.
        (Some((_, word)), _) => DIGIT_WORDS
            .iter()
            .position(|&w| w == word)
            .map(|pos| pos as u32),
        _ => None,
    }
}
