use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day06.txt").expect("Day 6 input file should be present.");
    let input: Vec<&str> = input.lines().collect();
    let sol1: u64 = product_of_ways_to_beat_records(&input);
    let sol2: u64 = ways_to_beat_record(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

// The input is two lines of text, each with a list of numbers, listing a series of records for a toy boat race.
// The first line is how long each race lasted (milliseconds), and the second line is the record distance (millimeters) for that race.
// The records are space-separated in the same order, so the first record in the first line is the time for the first distance record in the second line.
// A toy boat has two actions, charge and release. A charge action takes 1 millisecond, and increases the boat's speed by 1 millimeter per millisecond. A boat cannot move until the charge actions are complete.
// If a race last for 5 milliseconds, then there are 6 possibilities, from 0 to 5 charges. 0 charges means the boat never moved, and 5 charges means the boat moved 5 millimeters per millisecond, but that's the entire duration of the race, so the boat cannot move. With 3 charges, then the boat does not move for the first 3 milliseconds, and then moves 3 millimeters per millisecond for the remaining 2 milliseconds for a total of 6 millimeters.
// We want to find the number of ways to beat the record
fn product_of_ways_to_beat_records(input: &[&str]) -> u64 {
    let times: Vec<u64> = input[0]
        .split_whitespace()
        .skip(1) // skip the "Time:" label
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<u64> = input[1]
        .split_whitespace()
        .skip(1) // skip the "Distance:" label
        .map(|x| x.parse().unwrap())
        .collect();

    let mut product = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        let ways_to_beat_record = calculate_ways_to_beat_record(*time, *distance);
        product *= ways_to_beat_record;
    }

    product
}

fn calculate_ways_to_beat_record(time: u64, distance: u64) -> u64 {
    let optimal_charge = time / 2;
    let mut ways_to_beat_record = 0;

    for charge in 0..=optimal_charge {
        let distance_covered = charge * (time - charge);
        if distance_covered > distance {
            // Count this charge and its mirror charge if it's different
            ways_to_beat_record += if charge == optimal_charge && time % 2 == 0 {
                1
            } else {
                2
            };
        }
    }

    ways_to_beat_record
}

// In the second part, the input is the same, but we parse it differently just by concatening the numbers in each line into a single number.
// So instead of times of 7, 15, 30 and distances of 9, 40, 200, we have a time of 71530 and a distance of 940200 which we do the same calculation on (might need to optimize the calculation).
fn ways_to_beat_record(input: &[&str]) -> u64 {
    let time: u64 = input[0]
        .split_whitespace()
        .skip(1) // skip the "Time:" label
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: u64 = input[1]
        .split_whitespace()
        .skip(1) // skip the "Distance:" label
        .collect::<String>()
        .parse()
        .unwrap();

    calculate_ways_to_beat_record(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day06_test_input() -> &'static str {
        "Time:      7  15   30
        Distance:  9  40  200"
    }

    #[test]
    fn test_product_of_ways_to_beat_record() {
        let input = day06_test_input();
        // split into lines
        let input: Vec<&str> = input.lines().collect();
        assert_eq!(product_of_ways_to_beat_records(&input), 288);
    }

    #[test]
    fn test_ways_to_beat_record() {
        let input = day06_test_input();
        // split into lines
        let input: Vec<&str> = input.lines().collect();
        assert_eq!(ways_to_beat_record(&input), 71503);
    }
}
