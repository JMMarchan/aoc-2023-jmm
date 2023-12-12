use crate::{Solution, SolutionPair};
use hashbrown::HashMap;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day12.txt").expect("Day 12 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    let sol1: u64 = sum_of_arrangements(&lines);
    let sol2: u64 = sum_of_folded_arrangements(&lines);

    (Solution::from(sol1), Solution::from(sol2))
}

// Input is a list of rows, split into two parts by a space.
// The first part is a string of dots and question marks, and hashes. This represents springs which may be operational (.), unknown (?), or broken (#).
// The second part is a list of numbers separated by commas. These numbers represent groups of broken springs, where a group is a contiguous sequence of broken springs.
// For example ??.??.?# 1,1,2 has 3 groups of broken springs, 1, 1, and 2.
// Note that only one actual broken spring is shown. We have to find out where the other broken springs are.
// There are four possible ways to arrange these groups based on the unknown springs.
// Find the sum of the number of ways to arrange the groups for each line of input.
fn sum_of_arrangements(input: &[&str]) -> u64 {
    input.iter().map(|line| spring_arrangements(line)).sum()
}

fn sum_of_folded_arrangements(input: &[&str]) -> u64 {
    input
        .iter()
        .map(|line| folded_spring_arrangements(line))
        .sum()
}

// Folded arrangements are just lines where the springs and groups are replaced with five copies of themselves, with ? between each copy
// So .# 1 becomes .#?.#?.#?.#?.# 1,1,1,1,1
fn folded_spring_arrangements(line: &str) -> u64 {
    let mut parts = line.split(' ');
    let springs = parts.next().unwrap();
    let groups: Vec<usize> = parts
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut folded_springs = String::new();
    for i in 0..5 {
        folded_springs.push_str(springs);
        if i < 4 {
            folded_springs.push('?');
        }
    }

    let mut folded_groups = Vec::new();
    for _ in 0..5 {
        folded_groups.extend(groups.iter().cloned());
    }

    let arrangements = calculate_arrangements(&folded_springs.as_bytes(), &folded_groups);

    // print folded
    // println!("{} {:?}: {}", folded_springs, folded_groups, arrangements);
    arrangements as u64
}

fn spring_arrangements(line: &str) -> u64 {
    let mut parts = line.split(' ');
    let springs = parts.next().unwrap();
    let groups: Vec<usize> = parts
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let arrangements = calculate_arrangements(&springs.as_bytes(), &groups);

    // println!("{}: {}", line, arrangements);

    arrangements as u64
}

fn calculate_arrangements(springs: &[u8], groups: &[usize]) -> usize {
    // Use dynamic programming to calculate the number of arrangements.
    let mut memo: HashMap<(usize, usize, usize), usize> = HashMap::new();

    // When iterating through springs and groups, we need to keep track of the current spring and group.
    // However, index of spring and group is not enough
    // We also need to keep track of the length of the current group so that we can check if it matches the expected length.
    fn dp(
        springs: &[u8],
        groups: &[usize],
        spring_index: usize,
        group_index: usize,
        group_length: usize,
        memo: &mut HashMap<(usize, usize, usize), usize>,
    ) -> usize {
        // Check if we've already calculated this.
        if let Some(&result) = memo.get(&(spring_index, group_index, group_length)) {
            return result;
        }

        // Base case: End of springs
        if spring_index == springs.len() {
            // There are no groups of # left to place
            if group_index == groups.len() && group_length == 0 {
                return 1;
            }
            // There is one group of # left, and it is the correct length
            if group_index == groups.len() - 1 && group_length == groups[group_index] {
                return 1;
            }
            // Otherwise, either we haven't placed all the groups, or the last group is the wrong length.
            return 0;
        }

        // Otherwise, there are more springs to place.
        let mut arrangements = 0;
        let current_spring = springs[spring_index];

        // In all cases, we can treat ? like a . or a #.

        // Case 1: The current spring is operational with no current group
        // So place the current spring, and move on to the next spring.
        if (current_spring == b'.' || current_spring == b'?') && group_length == 0 {
            arrangements += dp(springs, groups, spring_index + 1, group_index, 0, memo);
        }

        // Case 2: The current spring is operational, and we have a current group
        // Check that there are more groups to place, and that the group is the expected length
        // Then we can move on to the next spring and the next group.
        if (current_spring == b'.' || current_spring == b'?') && group_length > 0 {
            if group_index < groups.len() && group_length == groups[group_index] {
                arrangements += dp(springs, groups, spring_index + 1, group_index + 1, 0, memo);
            }
        }

        // Case 3: Suppose the current spring is broken, so it must be part of a group.
        if current_spring == b'#' || current_spring == b'?' {
            arrangements += dp(
                springs,
                groups,
                spring_index + 1,
                group_index,
                group_length + 1,
                memo,
            );
        }

        memo.insert((spring_index, group_index, group_length), arrangements);
        arrangements
    }

    dp(springs, groups, 0, 0, 0, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#
    }

    #[test]
    fn test_spring_arrangements() {
        assert_eq!(spring_arrangements(". 1"), 0);
        assert_eq!(spring_arrangements("? 1"), 1);
        assert_eq!(spring_arrangements("?? 1"), 2);
        assert_eq!(spring_arrangements("??# 2"), 1);
        assert_eq!(spring_arrangements("???.### 1,1,3"), 1);
        assert_eq!(spring_arrangements("#?#.??? 1,1,2"), 2);
        assert_eq!(spring_arrangements(".??..??...?##. 1,1,3"), 4);
        assert_eq!(spring_arrangements("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(spring_arrangements("????.#...#... 4,1,1"), 1);
        assert_eq!(spring_arrangements("????.######..#####. 1,6,5"), 4);
        assert_eq!(spring_arrangements("?###???????? 3,2,1"), 10);
        assert_eq!(spring_arrangements(".?????...? 1,1,1"), 7);
    }

    #[test]
    fn test_spring_arrangements_edge_case() {
        assert_eq!(spring_arrangements("???#?? 1,1"), 3);
    }

    #[test]
    fn test_folded_spring_arrangements() {
        assert_eq!(folded_spring_arrangements(".??..??...?##. 1,1,3"), 16384);
    }
}
