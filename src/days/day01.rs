use crate::{Solution, SolutionPair};
use regex::Regex;
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
    let re = Regex::new(r"\d").unwrap();
    input
        .iter()
        .filter_map(|line| {
            let matches: Vec<_> = re.find_iter(line).collect();
            let first_digit = matches.first()?;
            let last_digit = matches.last()?;
            Some(
                format!("{}{}", first_digit.as_str(), last_digit.as_str())
                    .parse::<u32>()
                    .unwrap(),
            )
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

fn find_digit(line: &str, find_last: bool) -> Option<u32> {
    let re = Regex::new(r"(zero|one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let mut matches: Vec<_> = re
        .find_iter(line)
        .map(|m| (m.start(), m.as_str()))
        .collect();

    // Sort the matches by their start index
    matches.sort_by_key(|&(index, _)| index);

    let digit = if find_last {
        matches.last()
    } else {
        matches.first()
    };

    digit.and_then(|&(_, digit_str)| match digit_str {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => digit_str.parse().ok(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenate_and_sum() {
        let input = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        assert_eq!(concatenate_and_sum(&input), 142);
    }

    #[test]
    fn test_parse_concat_sum() {
        let input = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
            "oneighthree",
        ];
        assert_eq!(parse_concat_sum(&input), 294);
    }
}
