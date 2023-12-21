use crate::{Solution, SolutionPair};
use hashbrown::HashMap;
use itertools::Itertools;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day20.txt").expect("Day 20 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    let sol1 = product_low_high_pulses(&lines);
    let sol2 = min_button_presses(&lines, "rx");

    (Solution::from(sol1), Solution::from(sol2))
}

// Input is a line-separated of modules that process low and high pulses.
// They have a optional prefix type, followed by a name, "->", and a comma-separated list of names.
// Flip-flop modules "%" are either on or off. They start off.
// If receiving a high pulse, nothing happens.
// If receiving a low pulse, and it was off, it turns on and sends a high pulse.
// If receiving a low pulse, and it was on, it turns off and sends a high pulse.
// Conjunction modules "&" remember the type of the most recent pulse from each input module.
// They default to a low pulse. When a pulse is received, it first updates their memory for that input.
// Then, if all inputs are high in memory, it sends a low pulse; otherwise, it sends a high pulse.
// There is a single broadcaster module that sends the same pulse to all of its outputs.
// You control a button module (not in the input) that sends one low pulse to the broadcaster when pressed.
// After pushing the button, you must wait until all pulses have propagated through the system.
// Pulses are always processed in the order they are sent.
// Find the product of the total number of low pulses and high pulses after pressing the button 1000 times.
fn product_low_high_pulses(input: &[&str]) -> u64 {
    let mut system = initialize_system(input);
    let mut total_low = 0;
    let mut total_high = 0;

    for _ in 0..1000 {
        // println!("--- {}: button -{:?}-> broadcaster", i + 1, Pulse::Low);
        total_low += 1;

        let (low_pulses, high_pulses) = process_pulses(&mut system, &mut vec![], None);
        // println!("{}: low: {}, high: {}", i + 1, low_pulses, high_pulses);
        total_low += low_pulses;
        total_high += high_pulses;
    }

    // println!("Total low pulses: {}", total_low);
    // println!("Total high pulses: {}", total_high);

    total_low * total_high
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone)]
enum ModuleKind {
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
    Broadcaster,
}

#[derive(Debug, Clone)]
struct Module {
    module_kind: ModuleKind,
    outputs: Vec<String>,
}

#[derive(Clone)]
struct PulseMessage {
    source_module: String,
    target_module: String,
    pulse: Pulse,
}

type System = HashMap<String, Module>;

fn initialize_system(input: &[&str]) -> System {
    let mut system: System = HashMap::new();
    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();

    for line in input {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let module_def = parts[0];
        let outputs: Vec<String> = parts[1].split(", ").map(|s| s.to_string()).collect();

        let (module_kind, label) = match module_def.chars().next().unwrap() {
            '%' => (ModuleKind::FlipFlop(false), module_def[1..].to_string()),
            '&' => (
                ModuleKind::Conjunction(HashMap::new()),
                module_def[1..].to_string(),
            ),
            _ => (ModuleKind::Broadcaster, module_def.to_string()),
        };

        system.insert(
            label.clone(),
            Module {
                module_kind,
                outputs: outputs.clone(),
            },
        );

        for output in outputs {
            inputs.entry(output).or_default().push(label.clone());
        }
    }

    // println!("Inputs: {:?}", inputs);

    for (name, module) in system.iter_mut() {
        if let ModuleKind::Conjunction(memory) = &mut module.module_kind {
            if let Some(input_modules) = inputs.get(name) {
                *memory = input_modules
                    .iter()
                    .map(|m| (m.clone(), Pulse::Low))
                    .collect::<HashMap<String, Pulse>>();
            }
        }
    }

    system
}

fn process_pulses(
    system: &mut System,
    input_module_periods: &mut Vec<(String, Option<u64>)>,
    button_presses: Option<u64>,
) -> (u64, u64) {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(PulseMessage {
        source_module: "button".to_string(),
        target_module: "broadcaster".to_string(),
        pulse: Pulse::Low,
    });

    while let Some(PulseMessage {
        source_module,
        target_module,
        pulse,
    }) = queue.pop_front()
    {
        if let Some(module) = system.get_mut(&target_module) {
            match &mut module.module_kind {
                ModuleKind::FlipFlop(state) => {
                    if let Pulse::Low = pulse {
                        *state = !*state;
                        // if it was off, turn on and send high pulse, if it was on, turn off and send low pulse
                        let pulse = if *state { Pulse::High } else { Pulse::Low };

                        for output in &module.outputs {
                            if *state {
                                high_pulses += 1;
                            } else {
                                low_pulses += 1;
                            }
                            // println!("{} -{:?}-> {}", target_module, pulse, output);
                            queue.push_back(PulseMessage {
                                source_module: target_module.clone(),
                                target_module: output.clone(),
                                pulse: pulse.clone(),
                            });
                        }
                    }
                }
                ModuleKind::Conjunction(memory) => {
                    if let Some(m) = memory.get_mut(&source_module) {
                        *m = pulse.clone();
                    }
                    // println!("Memory: {:?}", memory);

                    let new_pulse = if memory.iter().all(|(_, p)| p == &Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };

                    for output in &module.outputs {
                        if new_pulse == Pulse::High {
                            high_pulses += 1;
                        } else {
                            low_pulses += 1;
                        }
                        // println!("{} -{:?}-> {}", target_module, new_pulse, output);
                        queue.push_back(PulseMessage {
                            source_module: target_module.clone(),
                            target_module: output.clone(),
                            pulse: new_pulse.clone(),
                        });
                    }
                }
                ModuleKind::Broadcaster => {
                    for output in &module.outputs {
                        low_pulses += 1;
                        // println!("{} -{:?}-> {}", target_module, pulse, output);
                        queue.push_back(PulseMessage {
                            source_module: target_module.clone(),
                            target_module: output.clone(),
                            pulse: Pulse::Low,
                        });
                    }
                }
            }
        }

        // Check if a high pulse is being sent by one of the input modules for the first time
        if let Some((_, period)) = input_module_periods
            .iter_mut()
            .find(|(module, _)| module == &source_module && pulse == Pulse::High)
        {
            println!("{} -{:?}-> {}", source_module, pulse, target_module);
            if period.is_none() {
                *period = button_presses;
            }
        }
    }

    (low_pulses, high_pulses)
}

// Find the minimum number of button presses required to send a single low pulse to rx
fn min_button_presses(input: &[&str], output_module: &str) -> u64 {
    // rx is the output of a single conjunction module, which itself takes some number of conjunction modules as inputs
    // &final -> rx
    //  &in1 -> final
    //  &in2 -> final
    // ...
    // We want a single low pulse to rx, and since final is a conjunction, we need all of its inputs to be high in memory.
    // This is infeasible to brute force. It will probably be some sort of lcm of the periods of the inputs.
    // (The example is suggestive of cycles, input probably has perfect cycles, ie no need for Chinese Remainder Theorem)

    let mut system: System = initialize_system(input);

    // Find the module that goes into rx
    let final_module = system
        .iter()
        .find(|(_, module)| module.outputs.contains(&output_module.to_string()))
        .unwrap()
        .0
        .clone();

    // Then get all of its inputs. Since it's a conjunction, it's in memory as a hashmap
    let inputs = match &system[&final_module].module_kind {
        ModuleKind::Conjunction(memory) => memory.keys().cloned().collect::<Vec<String>>(),
        _ => unreachable!(),
    };

    println!("Final module: {}", final_module);
    println!("Inputs: {:?}", inputs);

    // Find the period of each input
    let mut input_module_periods = inputs
        .iter()
        .map(|input| (input.to_string(), None))
        .collect_vec();

    let mut button_presses: u64 = 0;
    while input_module_periods.iter().any(|(_, val)| val.is_none()) {
        button_presses += 1;
        process_pulses(&mut system, &mut input_module_periods, Some(button_presses));
    }

    println!("Input module periods: {:?}", input_module_periods);

    lcm(&input_module_periods
        .iter()
        .filter_map(|(_, period)| *period)
        .collect_vec())
}

fn lcm(values: &[u64]) -> u64 {
    values.iter().fold(1, |acc, &x| acc * x / gcd(acc, x))
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#
    }

    fn test_input_2() -> &'static str {
        r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#
    }

    #[test]
    fn test_product_low_high_pulses() {
        let input1 = test_input().lines().collect::<Vec<&str>>();
        let sol1 = product_low_high_pulses(&input1);
        assert_eq!(sol1, 32000000);
        let input2 = test_input_2().lines().collect::<Vec<&str>>();
        let sol2 = product_low_high_pulses(&input2);
        assert_eq!(sol2, 11687500);
    }
}
