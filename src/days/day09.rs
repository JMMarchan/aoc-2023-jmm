use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day09.txt").expect("Day 9 input file should be present.");
    let input: Vec<&str> = input.lines().collect();
    let sol1: i64 = sum_of_extrapolated_values(&input, false);
    let sol2: i64 = sum_of_extrapolated_values(&input, true);

    (Solution::from(sol1), Solution::from(sol2))
}

// The input is list of space-separated sequences of integers (i64)
// For each sequence, we extrapolate the next number in the sequence by looking at the sequence of differences between each pair of numbers
// For example, the sequence 0 3 6 9 12 15 has differences 3 3 3 3 3. This difference sequence has differences 0 0 0 0, so the next number in the sequence is 18.
// The sequence 1 3 6 10 15 21 has differences 2 3 4 5 6, which has differences 1 1 1 1, which has differences 0 0 0, so the next number in the sequence is 28.
// In general, we can extrapolate the next number in the sequence by looking at the sequence of differences between each pair of numbers, and then looking at the sequence of differences between each pair of numbers in that sequence, and so on, until we reach a zero sequence, from which we can extrapolate the next number in the sequence.
// Find the sum of the extrapolated values for each sequence.
// In the second part, we extrapolate backward instead, getting the value before the first number in the sequence.
fn sum_of_extrapolated_values(input: &[&str], backward: bool) -> i64 {
    input
        .iter()
        .map(|line| {
            let mut numbers = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            if backward {
                numbers.reverse();
            }
            extrapolate_next(&numbers)
        })
        .sum()
}

// This function extrapolates the next number in the sequence by looking at the sequence of differences between each pair of numbers, and then looking at the sequence of differences between each pair of numbers in that sequence, and so on, until we reach a zero sequence, from which we can extrapolate the next number in the sequence, which is the last number in the sequence plus the extrapolated next number in the sequence of differences.
fn extrapolate_next(numbers: &[i64]) -> i64 {
    let diffs: Vec<i64> = numbers.windows(2).map(|pair| pair[1] - pair[0]).collect();

    if diffs.iter().all(|&x| x == 0) {
        *numbers.last().unwrap()
    } else {
        numbers.last().unwrap() + extrapolate_next(&diffs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day07_test_input() -> &'static str {
        "0 3 6 9 12 15\n\
1 3 6 10 15 21\n\
10 13 16 21 30 45"
    }

    #[test]
    fn test_sum_of_extrapolated_values() {
        let input = day07_test_input();
        // split into lines
        let input: Vec<&str> = input.lines().collect();
        assert_eq!(sum_of_extrapolated_values(&input, false), 114);
    }

    #[test]
    fn test_sum_of_extrapolated_values_backward() {
        let input = day07_test_input();
        // split into lines
        let input: Vec<&str> = input.lines().collect();
        // seq 1
        // 1. 0 3 6 9 12 15
        // 2.  3 3 3 3 3
        // adding 3 to the start of the constant sequence we have
        // 2. 3 3 3 3 3 3
        // 1. -3 0 3 6 9 12 15
        // so -3 is the extrapolated value
        // seq 2
        // 1. 1 3 6 10 15 21
        // 2.  2 3 4 5 6
        // 3.   1 1 1 1
        // adding 1 to the start of the constant sequence we have
        // 3. 1 1 1 1 1
        // 2. 1 2 3 4 5 6
        // 1. 0 1 3 6 10 15 21
        // so 0 is the extrapolated value
        // seq 3
        // 1. 10 13 16 21 30 45
        // 2.   3  3  5  9 15
        // 3.     0  2  4  6
        // 4.       2  2  2
        // adding 2 to the start of the constant sequence we have
        // 4. 2 2 2 2
        // 3. -2 0 2 4 6
        // 2. 5 3 3 5 9 15
        // 1. 5 10 13 16 21 30 45
        // so 5 is the extrapolated value
        assert_eq!(sum_of_extrapolated_values(&input, true), 2);
        assert_eq!(sum_of_extrapolated_values(&input, true), 0);
    }
}
