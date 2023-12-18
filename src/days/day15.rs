use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, hash};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day15.txt").expect("Day 15 input file should be present");
    let sol1: u64 = sum_hash_algorithm(&input);
    let sol2: u64 = initialization_sequence(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

// The HASH algorithm turns any string of characters into a integer 0-255
// To run the algorithm, start with a current value of zero
// For each character in the string starting from the beginning:
// 1. determine the ascii code of the character
// 2. Add the ascii code to the current value
// 3. Set the current value to itself multiplied by 17
// 4. Set current value to remainder of dividing current value by 256
// ie: current_value = (current_value + ascii_value) * 17 % 256
// The input is the init sequence, a comma-separated list of strings (ignore newlines)
// For each string, run the HASH algorithm and get the result
// Find the sum of the results
fn sum_hash_algorithm(input: &str) -> u64 {
    // remove newlines, split on commas, and run the hash algorithm on each string
    input.trim().split(',').map(|s| hash_algorithm(s)).sum()
}

fn hash_algorithm(input: &str) -> u64 {
    let mut current_value = 0;
    for c in input.chars() {
        let ascii_value = c as u64;
        current_value = (current_value + ascii_value) * 17 % 256;
    }
    current_value
}

// There exists 256 boxes, numbered 0-255. The boxes are arranged in a line starting at box 0 where the light enters the line, then box 1, then box 2, etc.
// In each box, there are lens slots that keep a lens position to focus light passing through the box
// There are lenses from focal length 1 to 9
// There is a sequence of steps in the initialization sequence called HASHMAP
// Each step begins with a sequence of letters that indicate the label of the lens on which the steo ioerates
// The result of running the HASH algorithm on the label indicates the correct box for that step
// The label will be followed by a character that indicates the oepration, either equals = or dash -
// If the operation is dash -, go to the relevant box and remove the lens with the given label if it exists
// Then move any remaining lenses as far forward in the box as they can go without changing their order, filling any space made by removing the indicated lens
// If the operation is equals =, it will be followed by the focal length of the lens that needs to go into the relevant box, giving it the given label
// If there is already a lens in the box with the given label, replace it with the new lens
// If there is not a lens in the box with the given label, add the lens to the box immediately behind any other lenses in the box
// The focusing power of a lens is the result of multiplying
// - One plus the box number
// - The slot number of the lens in the box, so the first lens in the box has slot number 1, the second lens in the box has slot number 2, etc
// - The focal length of the lens
// Find the sum of the focusing power of all lenses in all boxes
#[derive(Debug)]
struct Box {
    lenses: Vec<Lens>,
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u8,
}

fn initialization_sequence(input: &str) -> u64 {
    // Create the boxes
    let mut boxes: Vec<Box> = (0..256).map(|_| Box { lenses: vec![] }).collect();

    input.trim().split(',').for_each(|step_str| {
        // This is a step in the initialization sequence
        // Get the letters, then the operation, then the focal length if applicable
        let (label, rest) =
            step_str.split_at(step_str.find(|c: char| !c.is_alphabetic()).unwrap_or(0));
        let (operation, rest) = rest.split_at(1);
        let focal_length: u8 = rest.parse().unwrap_or(0);
        // println!(
        //     "label: {}, operation: {}, focal_length: {}",
        //     label, operation, focal_length
        // );

        // Run the hash algorithm on the label to get the box number
        let box_number = hash_algorithm(label) as usize;

        let r#box = &mut boxes[box_number];
        match operation {
            "-" => {
                r#box.lenses.retain(|lens| lens.label != label);
            }
            "=" => {
                // Replace or add the lens with the given label
                let lens_index = r#box.lenses.iter().position(|lens| lens.label == label);
                if let Some(index) = lens_index {
                    r#box.lenses[index] = Lens {
                        label: label.to_string(),
                        focal_length,
                    };
                } else {
                    r#box.lenses.push(Lens {
                        label: label.to_string(),
                        focal_length,
                    });
                }
            }
            _ => panic!("Invalid operation"),
        }

        // print all non empty boxes
        // for (i, box_to_print) in boxes.iter().enumerate() {
        //     if !box_to_print.lenses.is_empty() {
        //         println!("Box {}: {:?}", i, box_to_print);
        //     }
        // }
    });

    boxes
        .iter()
        .enumerate()
        .map(|(i, box_to_print)| {
            // calculate the focusing power of all lenses in the box
            box_to_print
                .lenses
                .iter()
                .enumerate()
                .map(|(j, lens)| (i + 1) * (j + 1) * lens.focal_length as usize)
                .sum::<usize>() as u64
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_hash_algorithm() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(sum_hash_algorithm(&input), 1320);
    }

    #[test]
    fn test_initialization_sequence() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(initialization_sequence(&input), 145);
    }
}
