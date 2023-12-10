use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_to_string("input/day05.txt").expect("Day 5 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    let sol1 = lowest_location_number(&lines, false);
    let sol2 = lowest_location_number(&lines, true);

    (Solution::from(sol1), Solution::from(sol2))
}

// The input lists the seeds that need to be planted.
// It also lists the soil, fertilizer, water, light, temperature, humidity, and location for each seed.
// Every type of seed, soil, etc, has a number associated, but numbers are not necessarily unique between types.
// The input starts with a list of seeds, each seed is a number.
// The rest of the input contains a list of maps that describe how to convert numbers from a source to a destination.
// For example, the first map is the seed-to-soil map, which describes how to convert seed numbers to soil numbers.
// The maps describe ranges of numbers that can be converted: the destination range start, the source range start, and the range length
// So for example, 50 96 4 means that the source range 50-53 maps to the destination range 96-99.
// Any source numbers outside of the source range are mapped to the same destination number, so for example 10 maps to 10.
// The seed-to-soil map is followed by the soil-to-fertilizer map, the fertilizer-to-water map, the water-to-light map, the light-to-temperature map, the temperature-to-humidity map, and the humidity-to-location map.
// Find the lowest location number corresponding to any of the seeds.
fn lowest_location_number(input: &[&str], range_based: bool) -> u64 {
    // First, we need to parse the input, ie get the seeds, and the maps.
    // The seeds are the first line, so we can just split the input into lines, and take the first line.
    // The maps are the rest of the lines. We separate them into maps by splitting on blank lines.
    // Then, we don't need the first line of each map, so we skip it.
    // We create a hashmap for each set of map, where the key is the source number, and the value is the destination number.

    let maps = input[1..]
        .split(|line| line.is_empty())
        .filter(|lines| !lines.is_empty()) // Filter out empty line groups
        .map(|lines| {
            let mut range_map = RangeMap::new();
            for line in lines.iter().skip(1) {
                let (destination_range_start, source_range_start, range_length) =
                    parse_map_line(line);
                range_map.add_mapping(source_range_start, destination_range_start, range_length);
            }
            range_map.sort_mappings();
            range_map
        })
        .collect::<Vec<RangeMap>>();

    if range_based {
        let ranges = parse_ranges(input[0]);
        process_with_boundaries(ranges, maps)
    } else {
        let seeds = parse_single_seeds(input[0]);
        process_seeds(seeds, maps)
    }
}

fn parse_map_line(line: &str) -> (u64, u64, u64) {
    let parts: Vec<u64> = line
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    (parts[0], parts[1], parts[2])
}

fn parse_single_seeds(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .skip(1) // skip the "seeds:" part
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
}

fn process_seeds(seeds: Vec<u64>, maps: Vec<RangeMap>) -> u64 {
    seeds
        .iter()
        .map(|&seed| maps.iter().fold(seed, |acc, map| map.get_destination(acc)))
        .min()
        .unwrap()
}

// In the second part, the seeds are not just a list of numbers, but a list of ranges, given by pairs of start and length.
// So seeds: 79 14 55 13 means that the seeds are 79-92 and 55-67.
fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .split_whitespace()
        .skip(1) // skip the "seeds:" part
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|chunk| {
            let start = chunk[0].parse::<u64>().unwrap();
            let length = chunk[1].parse::<u64>().unwrap();
            (start, start + length - 1)
        })
        .collect()
}

// The amount of seeds involved is too large to process them all, so we need to find a way to reduce the number of seeds.
// Instead of processing all seeds, we only transform the boundaries of the seed ranges, then find the lowest location number among the transformed boundaries.
fn process_with_boundaries(ranges: Vec<(u64, u64)>, mut maps: Vec<RangeMap>) -> u64 {
    // Fill gaps in each map
    for map in &mut maps {
        map.fill_gaps();
    }
    let mut current_ranges = ranges;
    for map in maps {
        let mut next_ranges = Vec::new();

        for range in current_ranges {
            let adjusted_ranges = map.adjust_range(range);
            for adjusted_range in adjusted_ranges {
                let adjusted_start = map.get_destination(adjusted_range.0);
                let adjusted_end = map.get_destination(adjusted_range.1);
                next_ranges.push((adjusted_start, adjusted_end));
            }
        }

        // remove duplicate ranges
        next_ranges.sort();
        next_ranges.dedup();

        current_ranges = next_ranges;
    }

    // Find the minimum of between the start and end points of the final ranges
    current_ranges
        .into_iter()
        .map(|range| range.0.min(range.1))
        .min()
        .unwrap()
}

#[derive(Debug, Clone)]
struct Range {
    source_range_start: u64,
    destination_range_start: u64,
    range_length: u64,
}

#[derive(Debug)]
struct RangeMap {
    mappings: Vec<Range>,
}

impl RangeMap {
    fn new() -> Self {
        RangeMap {
            mappings: Vec::new(),
        }
    }

    fn add_mapping(
        &mut self,
        source_range_start: u64,
        destination_range_start: u64,
        range_length: u64,
    ) {
        self.mappings.push(Range {
            source_range_start,
            destination_range_start,
            range_length,
        });
    }

    fn sort_mappings(&mut self) {
        self.mappings.sort_by_key(|range| range.source_range_start);
    }

    fn get_destination(&self, source: u64) -> u64 {
        let index = match self
            .mappings
            .binary_search_by_key(&source, |range| range.source_range_start)
        {
            Ok(index) => {
                return self.mappings[index].destination_range_start
                    + (source - self.mappings[index].source_range_start)
            }
            Err(index) => index,
        };

        if index == 0 {
            return source; // Source is smaller than the first range start
        }

        let range = &self.mappings[index - 1];
        if source < range.source_range_start + range.range_length {
            return range.destination_range_start + (source - range.source_range_start);
        }

        source // Source is outside any defined range
    }

    // Method to fill gaps with zero-shift ranges
    fn fill_gaps(&mut self) {
        // Assuming the maps don't have overlapping ranges
        // Add initial range if necessary
        if let Some(first_range) = self.mappings.first() {
            if first_range.source_range_start > 0 {
                self.mappings.insert(
                    0,
                    Range {
                        source_range_start: 0,
                        destination_range_start: 0,
                        range_length: first_range.source_range_start,
                    },
                );
            }
        }

        // Fill gaps between existing ranges
        let mut i = 0;
        while i < self.mappings.len() - 1 {
            let current_end = self.mappings[i].source_range_start + self.mappings[i].range_length;
            let next_start = self.mappings[i + 1].source_range_start;

            if current_end < next_start {
                self.mappings.insert(
                    i + 1,
                    Range {
                        source_range_start: current_end,
                        destination_range_start: current_end,
                        range_length: next_start - current_end,
                    },
                );
            }

            i += 1;
        }

        // Add final range if necessary
        let last_range_end = self.mappings.last().unwrap().source_range_start
            + self.mappings.last().unwrap().range_length;
        self.mappings.push(Range {
            source_range_start: last_range_end,
            destination_range_start: last_range_end,
            range_length: u64::MAX - last_range_end,
        });
    }

    fn adjust_range(&self, input_range: (u64, u64)) -> Vec<(u64, u64)> {
        let (input_start, input_end) = input_range;
        let mut adjusted_ranges = Vec::new();

        for range in &self.mappings {
            let range_end = range.source_range_start + range.range_length - 1;

            // Check for complete or partial overlap
            if input_end >= range.source_range_start && input_start <= range_end {
                let adjusted_start = input_start.max(range.source_range_start);
                let adjusted_end = input_end.min(range_end);
                adjusted_ranges.push((adjusted_start, adjusted_end));
            }
        }

        if adjusted_ranges.is_empty() {
            // If no overlap, the input range is retained as is
            adjusted_ranges.push(input_range);
        }

        adjusted_ranges
    }
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
