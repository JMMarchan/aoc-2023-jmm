use crate::{Solution, SolutionPair};
use rayon::prelude::*;
use regex::Regex;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day01.txt").expect("Day 1 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    let sol1 = concatenate_and_sum(&lines, false);
    let sol2 = concatenate_and_sum(&lines, true);

    (Solution::from(sol1), Solution::from(sol2))
}

fn concatenate_and_sum(input: &[&str], parse_digit_words: bool) -> u32 {
    input
        .par_iter()
        .map(|line| {
            let (first_digit, last_digit) = find_first_last_digit(line, parse_digit_words);
            format!("{:?}{:?}", first_digit, last_digit)
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

const DIGIT_WORDS: [&'static str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_first_last_digit(line: &str, parse_digit_words: bool) -> (usize, usize) {
    // Find the indices of the first and last numerical digits in the line.
    let first_digit_index = line.find(|c: char| c.is_digit(10));
    let last_digit_index = line.rfind(|c: char| c.is_digit(10));

    let mut first_digit: Option<usize> = None;
    let mut last_digit: Option<usize> = None;

    // If looking for digit words, find the first and last digit words in the line.
    if parse_digit_words {
        // Get the first digit word in the line.
        let first_digit_word = DIGIT_WORDS
            .iter()
            .enumerate()
            .map(|(num, &word)| {
                // (Option<usize>, &str, usize)
                // index of word in line, word, number (as index of DIGIT_WORDS)
                (line.find(word), word, num)
            })
            .filter(|(index, _, _)| index.is_some())
            .min_by_key(|(index, _, _)| index.unwrap());
        // If we find the first digit word, and either we haven't found the first digit index yet,
        // or the digit word comes before the first digit index, then use the digit word.
        if let Some((index, _, num)) = first_digit_word {
            if first_digit_index.is_none() || index.unwrap() < first_digit_index.unwrap() {
                first_digit = Some(num);
            }
        }

        // Get the last digit word in the line.
        let last_digit_word = DIGIT_WORDS
            .iter()
            .enumerate()
            .map(|(num, &word)| (line.rfind(word), word, num))
            .filter(|(index, _, _)| index.is_some())
            .max_by_key(|(index, _, _)| index.unwrap());
        // Use digit word if it comes later
        if let Some((index, _, num)) = last_digit_word {
            if last_digit_index.is_none() || index.unwrap() > last_digit_index.unwrap() {
                last_digit = Some(num);
            }
        }
    }

    // If we didn't find any digit words before the first digit index
    // (maybe because we're not looking for digit words), then use the first digit index.
    if first_digit_index.is_some() && first_digit.is_none() {
        let digit_char = line.chars().nth(first_digit_index.unwrap()).unwrap();
        first_digit = Some(digit_char.to_digit(10).unwrap() as usize);
    }

    if last_digit_index.is_some() && last_digit.is_none() {
        let digit_char = line.chars().nth(last_digit_index.unwrap()).unwrap();
        last_digit = Some(digit_char.to_digit(10).unwrap() as usize);
    }

    (first_digit.unwrap(), last_digit.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_last_digit() {
        let input = "two1nine";
        assert_eq!(find_first_last_digit(input, false), (1, 1));
        assert_eq!(find_first_last_digit(input, true), (2, 9));

        let input = "oneight";
        assert_eq!(find_first_last_digit(input, true), (1, 8));

        let input = "34onefive98";
        assert_eq!(find_first_last_digit(input, false), (3, 8));
        assert_eq!(find_first_last_digit(input, true), (3, 8));

        let input = "two";
        assert_eq!(find_first_last_digit(input, true), (2, 2));
    }

    #[test]
    fn test_concatenate_and_sum() {
        let input = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        assert_eq!(concatenate_and_sum(&input, false), 142);
    }
}
