use crate::{Solution, SolutionPair};
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
    fs::read_to_string,
};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_to_string("input/day05.txt").expect("Day 5 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    let sol1 = lowest_location_number(&lines, false);
    let sol2 = lowest_location_number(&lines, true);

    (Solution::from(sol1), Solution::from(sol2))
}

// A Map is essentially a piecewise function from u64 to u64.
// All we need to store is the start of each range, and the shift for that range. The end of a range is the start of the next range.
#[derive(Debug)]
struct Map {
    mappings: Vec<RangeShift>,
}

#[derive(Debug)]
struct RangeShift {
    range_start: u64,
    shift: i64,
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // A mapping looks like [start, end) -> [start + shift, end + shift)
        write!(f, "Mappings:\n")?;
        let mut mappings = self.mappings.iter();
        let mut current_mapping = mappings.next();
        while let Some(mapping) = current_mapping {
            let next_mapping = mappings.next();
            match next_mapping {
                Some(next_mapping) => writeln!(
                    f,
                    "[{}, {}) -> [{}, {}), Shift: {}",
                    mapping.range_start,
                    next_mapping.range_start,
                    mapping.range_start as i64 + mapping.shift,
                    next_mapping.range_start as i64 + mapping.shift,
                    mapping.shift
                )?,
                None => write!(
                    f,
                    "[{}, u64::MAX) -> [{}, u64::MAX), Shift: 0",
                    mapping.range_start, mapping.range_start
                )?,
            }
            current_mapping = next_mapping;
        }
        Ok(())
    }
}

impl Map {
    fn new() -> Self {
        Map {
            mappings: vec![RangeShift {
                range_start: 0,
                shift: 0,
            }],
        }
    }

    fn add_range_shift(
        &mut self,
        source_range_start: u64,
        destination_range_start: u64,
        range_length: u64,
    ) {
        // We can calculate the shift from the source and destination range starts
        // The shift is the difference between the destination range start and the source range start
        // We also add a second mapping for the end of the range, which is the start of the next range, only if there isn't already a mapping for the end of the range
        // for example, if the paramters are 50, 96, 4, then the first range shift has start 50 with shift 46, and the second range shift has start 54 with shift 0. Thus numbers 50-53 map to 96-99, and number 54 maps to 54.
        // Let's say we call this function again with the parameters are 45, 55, 5, then the first range shift has start 45 with shift 10; however, the second range shift that would start at 50 already exists, so we don't add it. Thus numbers 45-49 map to 55-59.
        let shift: i64 = destination_range_start as i64 - source_range_start as i64;
        let new_shift = RangeShift {
            range_start: source_range_start,
            shift,
        };
        let end = source_range_start + range_length;
        let end_shift = RangeShift {
            range_start: end,
            shift: 0,
        };

        // Insert the new shift in the sorted position, replacing if the existing shift is 0
        let pos = self.mappings.binary_search_by(|probe| {
            if probe.range_start < new_shift.range_start {
                Ordering::Less
            } else if probe.range_start > new_shift.range_start {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        match pos {
            Ok(pos) if self.mappings[pos].shift == 0 => self.mappings[pos] = new_shift,
            Ok(_) => (), // The shift already exists and is not 0, do nothing
            Err(pos) => self.mappings.insert(pos, new_shift),
        }

        // Insert the end shift in the sorted position, replacing if the existing shift is 0
        let pos = self.mappings.binary_search_by(|probe| {
            if probe.range_start < end_shift.range_start {
                Ordering::Less
            } else if probe.range_start > end_shift.range_start {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        match pos {
            Ok(_) => (), // The shift already exists and is not 0, do nothing
            Err(pos) => self.mappings.insert(pos, end_shift),
        }
    }

    fn get_destination(&self, source: u64) -> u64 {
        // We can find the destination by finding the range shift that contains the source, and adding the shift to the source. The way we're storing range shifts, the range shift that contains the source is the one with the largest range start that is less than or equal to the source.
        // All numbers u64 are in some range shift because the last range shift will be some n with shift 0, and all numbers greater than or equal to n will map to themselves.
        let pos = self.mappings.binary_search_by(|probe| {
            if probe.range_start < source {
                Ordering::Less
            } else if probe.range_start > source {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        match pos {
            Ok(pos) => (source as i64 + self.mappings[pos].shift) as u64,
            Err(pos) => (source as i64 + self.mappings[pos - 1].shift) as u64,
        }
    }

    fn transform_range(&self, range_start: u64, range_end: u64) -> Vec<(u64, u64)> {
        // find the largest range shift that is less than or equal to the range start
        // find the largest range shift that is less than or equal to the range end
        // this may encompass one or multiple range shifts
        // if it's just one, the vector will have one element, a tuple of the range start and range end run through the get_destination function
        // if it's multiple, the vector will have multiple elements, each a tuple of the range start and range end run through the get_destination function. the number of elements will be the number of range shifts that overlapped by the range start and range end
        // for example, if the rangeshifts are on [0, 20), [20, 30), [30, 50), and the input range is [10, 40], then the output vector will have three elements: [10, 20], [21, 30], [31, 40] whose endpoints are run through the get_destination function
        let mut transformed_ranges = Vec::new();

        // Use binary search to find the first relevant mapping
        let start_index = match self
            .mappings
            .binary_search_by(|probe| probe.range_start.cmp(&range_start))
        {
            Ok(index) => index,
            Err(index) => index,
        };

        // This loop splits the range into multiple ranges, each of which is transformed by a single mapping.
        for (i, mapping) in self.mappings.iter().enumerate().skip(start_index - 1) {
            // We've processed all relevant mappings
            if mapping.range_start > range_end {
                break;
            }

            // Skip the current mapping because it doesn't affect the range
            if i < self.mappings.len() - 1 && self.mappings[i + 1].range_start <= range_start {
                continue;
            }

            // Calculate the start and end of the transformed range.
            // The start is the maximum of the start of the current mapping and the start of the range.
            // The end is the minimum of the start of the next mapping and the end of the range,
            // or the end of the range if there is no next mapping.
            let start = std::cmp::max(mapping.range_start, range_start);
            let end = if i < self.mappings.len() - 1 {
                std::cmp::min(self.mappings[i + 1].range_start, range_end)
            } else {
                range_end
            };

            // Apply the shift to the start and end of the range to get the transformed range.
            let transformed_start = (start as i64 + mapping.shift) as u64;
            let transformed_end = (end as i64 + mapping.shift) as u64;

            // Add the transformed range to the vector.
            transformed_ranges.push((transformed_start, transformed_end));
        }

        transformed_ranges
    }
}

fn lowest_location_number(input: &[&str], range_based: bool) -> u64 {
    let maps = input[1..]
        .split(|line| line.is_empty())
        .filter(|lines| !lines.is_empty()) // Filter out empty line groups
        .map(|lines| {
            let mut map = Map::new();
            for line in lines.iter().skip(1) {
                let (destination_range_start, source_range_start, range_length) =
                    parse_map_line(line);
                map.add_range_shift(source_range_start, destination_range_start, range_length);
            }
            map
        })
        .collect::<Vec<Map>>();

    if range_based {
        let ranges = input[0]
            .split_whitespace()
            .skip(1)
            .collect::<Vec<&str>>()
            .chunks(2)
            .map(|chunk| {
                let start = chunk[0].parse::<u64>().unwrap();
                let length = chunk[1].parse::<u64>().unwrap();
                (start, start + length - 1)
            })
            .collect::<Vec<(u64, u64)>>();

        let mut current_ranges = ranges;

        // for each map, for each range, we transform the range, which may split it into multiple ranges if it spans multiple rangeshifts
        for map in &maps {
            let mut next_ranges = Vec::new();
            for range in &current_ranges {
                let transformed_ranges = map.transform_range(range.0, range.1);
                for transformed_range in transformed_ranges {
                    next_ranges.push(transformed_range);
                }
            }
            // println!("{}", map);
            // println!("Current: {:?}", current_ranges);
            // println!("Next: {:?}", next_ranges);
            current_ranges = next_ranges;
        }

        // find the minimum of the start and end points of the final ranges
        current_ranges
            .into_iter()
            .map(|range| range.0.min(range.1))
            .min()
            .unwrap()
    } else {
        let seeds = input[0]
            .split_whitespace()
            .skip(1)
            .filter_map(|s| s.parse::<u64>().ok())
            .collect::<Vec<u64>>();

        let mut current_seeds = seeds;
        for map in &maps {
            let mut next_seeds = Vec::new();
            for seed in &current_seeds {
                next_seeds.push(map.get_destination(*seed));
            }
            // println!("{}", map);
            // println!("Current: {:?}", current_seeds);
            // println!("Next: {:?}", next_seeds);
            current_seeds = next_seeds;
        }

        *current_seeds.iter().min().unwrap()
    }
}

fn parse_map_line(line: &str) -> (u64, u64, u64) {
    let parts: Vec<u64> = line
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    (parts[0], parts[1], parts[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day05_test_input() -> &'static str {
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
    }

    #[test]
    fn test_lowest_location_number() {
        let input = day05_test_input();
        // split into lines
        let input: Vec<&str> = input.lines().collect();
        assert_eq!(lowest_location_number(&input, false), 35);
    }

    #[test]
    fn test_lowest_location_number_range_based() {
        let input = day05_test_input();
        // split into lines
        let input: Vec<&str> = input.lines().collect();
        assert_eq!(lowest_location_number(&input, true), 46);
    }
}
