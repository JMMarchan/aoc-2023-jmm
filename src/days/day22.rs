use crate::{Solution, SolutionPair};
use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day22.txt").expect("Day 22 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    let sol1 = safe_bricks(&lines);
    let sol2 = sum_bricks_falling(&lines);

    (Solution::from(sol1), Solution::from(sol2))
}

// The input is a line separated list of bricks, given by two x,y,z coordinates separated by tilde.
// These coordinates represent the ends of the brick. 2,2,2~2,2,2 is a 1x1x1 brick, while 0,0,10~1,0,10 is a 2x1x1 brick.
// 0,0,1~0,0,10 is a 1x1x10 brick. The ground is at z=0, so the lowest z value a brick can have is 1.
// The given list of bricks has some bricks in the air, and we need to find out where they all end up after they fall.
// Bricks never rotate despite physics, so they will always fall straight down.
// A brick is safe to disintegrate if, after removing it, no other bricks will fall.
// Find the number of bricks that are safe to disintegrate.
fn safe_bricks(input: &[&str]) -> usize {
    // First, we need to simulate the bricks falling.
    // We do this by getting lowest z value for each brick and then sorting by that.
    // Then we iterate through the bricks, and for each brick, find the highest z value of the bricks below it.
    // Every brick is 1x1xn. A brick is below another brick if the x values and the y values overlap, and the z value is lower.
    // Every brick is formatted such that x1 <= x2, y1 <= y2, and z1 <= z2.
    // So we just sort on z1, and then for each brick, find the highest z2 of the bricks below it.

    let sorted_bricks = sort_bricks(input);

    // Also need to consider if there are no bricks below it, then move to z=1.
    // How to represent movement? Just change the z1 and z2 values.

    let fallen_bricks = fall_bricks(&sorted_bricks);

    // After that, we can find safe bricks by checking if, for all bricks directly above it
    // there exists at least two bricks that are not directly above it.

    let (bricks_directly_above, bricks_directly_below) =
        bricks_directly_above_and_below(&fallen_bricks);

    bricks_directly_above
        .iter()
        .map(|(&brick, directly_above)| {
            directly_above.iter().all(|&other| {
                let directly_below = bricks_directly_below.get(&other).unwrap();
                directly_below.len() >= 2
            })
        })
        .filter(|&b| b)
        .count()
}

fn sort_bricks(input: &[&str]) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = input.iter().map(|&s| Brick::from_str(s)).collect();
    bricks.sort_by_key(|b| b.z1);
    bricks
}

fn fall_bricks(sorted_bricks: &[Brick]) -> Vec<Brick> {
    let mut fallen_bricks = sorted_bricks.to_vec();
    for i in 0..sorted_bricks.len() {
        let mut new_brick = sorted_bricks[i].clone();
        let max_z2 = fallen_bricks
            .iter()
            .filter(|&other| other.is_below(&new_brick))
            .map(|other| other.z2)
            .max()
            .unwrap_or(0);

        new_brick.z1 = max_z2 + 1;
        new_brick.z2 = max_z2 + sorted_bricks[i].height();
        fallen_bricks[i] = new_brick;
    }

    fallen_bricks
}

fn bricks_directly_above_and_below(
    fallen_bricks: &[Brick],
) -> (BricksDirectlyAbove, BricksDirectlyBelow) {
    let mut bricks_directly_above: BricksDirectlyAbove = HashMap::new();
    let mut bricks_directly_below: BricksDirectlyBelow = HashMap::new();
    for brick in fallen_bricks.iter() {
        let directly_above: Vec<Brick> = fallen_bricks
            .iter()
            .filter(|&other| brick.is_directly_below(other))
            .map(|&other| other)
            .collect();
        bricks_directly_above.insert(*brick, directly_above.clone());
        for &other in directly_above.iter() {
            if let Some(directly_below) = bricks_directly_below.get_mut(&other) {
                directly_below.push(*brick);
            } else {
                bricks_directly_below.insert(other, vec![*brick]);
            }
        }
    }

    (bricks_directly_above, bricks_directly_below)
}

// For each brick, find how many other bricks would fall if it were removed.
fn sum_bricks_falling(input: &[&str]) -> usize {
    let sorted_bricks = sort_bricks(input);
    let fallen_bricks = fall_bricks(&sorted_bricks);
    let (bricks_directly_above, bricks_directly_below) =
        bricks_directly_above_and_below(&fallen_bricks);

    let mut total_falling_bricks = 0;

    // TODO: could make more efficient with memoization from top down
    for &brick in fallen_bricks.iter() {
        let mut count_falling = 0;
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(brick);

        while let Some(current_brick) = queue.pop_front() {
            if visited.insert(current_brick) {
                let vec: Vec<Brick> = vec![];
                let above_bricks = bricks_directly_above.get(&current_brick).unwrap_or(&vec);

                for &above_brick in above_bricks {
                    let below_bricks = bricks_directly_below.get(&above_brick).unwrap_or(&vec);

                    // If removing the current brick causes above_brick to fall
                    if below_bricks.iter().all(|&b| visited.contains(&b)) {
                        queue.push_back(above_brick);
                        count_falling += 1;
                    }
                }
            }
        }

        total_falling_bricks += count_falling;
    }

    total_falling_bricks
}

type BricksDirectlyAbove = HashMap<Brick, Vec<Brick>>;
type BricksDirectlyBelow = HashMap<Brick, Vec<Brick>>;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Brick {
    x1: usize,
    y1: usize,
    z1: usize,
    x2: usize,
    y2: usize,
    z2: usize,
}

impl std::fmt::Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Brick ({}, {}, {}), ({}, {}, {})",
            self.x1, self.y1, self.z1, self.x2, self.y2, self.z2
        )
    }
}

impl Brick {
    fn new(x1: usize, y1: usize, z1: usize, x2: usize, y2: usize, z2: usize) -> Self {
        Brick {
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
        }
    }

    fn from_str(s: &str) -> Self {
        let mut coords = s.split('~');
        let mut start_coords = coords.next().unwrap().split(',');
        let mut end_coords = coords.next().unwrap().split(',');

        let x1 = start_coords.next().unwrap().parse().unwrap();
        let y1 = start_coords.next().unwrap().parse().unwrap();
        let z1 = start_coords.next().unwrap().parse().unwrap();

        let x2 = end_coords.next().unwrap().parse().unwrap();
        let y2 = end_coords.next().unwrap().parse().unwrap();
        let z2 = end_coords.next().unwrap().parse().unwrap();

        Brick::new(x1, y1, z1, x2, y2, z2)
    }

    fn is_below(&self, other: &Self) -> bool {
        let is_below = self.z2 < other.z1;
        let x_overlap = self.x1 <= other.x2 && self.x2 >= other.x1;
        let y_overlap = self.y1 <= other.y2 && self.y2 >= other.y1;

        is_below && x_overlap && y_overlap
    }

    fn is_directly_below(&self, other: &Self) -> bool {
        let is_below = self.z2 + 1 == other.z1;
        let x_overlap = self.x1 <= other.x2 && self.x2 >= other.x1;
        let y_overlap = self.y1 <= other.y2 && self.y2 >= other.y1;

        is_below && x_overlap && y_overlap
    }

    fn height(&self) -> usize {
        self.z2 - self.z1 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#
    }

    #[test]
    fn test_safe_bricks() {
        let lines: Vec<&str> = test_input().lines().collect();
        let sol1 = safe_bricks(&lines);
        assert_eq!(sol1, 5);
    }

    #[test]
    fn test_sum_bricks_falling() {
        let lines: Vec<&str> = test_input().lines().collect();
        let sol2 = sum_bricks_falling(&lines);
        assert_eq!(sol2, 7);
    }
}
