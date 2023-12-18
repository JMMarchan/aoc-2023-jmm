#![allow(unused_imports)]
mod days;
mod etc;

use chrono::prelude::*;
use days::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25,
};
use etc::solution::Solution;
use hashbrown::HashMap;
use itertools::Itertools;
use std::env;
use std::time::Instant;

pub type SolutionPair = (Solution, Solution);

fn main() {
    let args: Vec<String> = env::args().collect();
    // If no arguments are given, try to find the latest day with a non-zero solution pair.
    if args.len() < 2 {
        // Check what day of December it is (ie, EST since problems come out at EST) and try that day.
        // If it's after the 25th, then try the 25th.
        // If the solution pair is zero, try the previous day until you find a non-zero solution pair.
        let now = Local::now();
        let later = Local.with_ymd_and_hms(2023, 12, 25, 0, 0, 0).unwrap();
        let day = if now > later { 25 } else { now.day() };

        let mut current_day = day;
        let mut p1;
        let mut p2;
        let mut elapsed_ms;

        loop {
            if current_day < day {
                println!(
                    "Day {} solution is zero, trying previous day {}...",
                    current_day + 1,
                    current_day
                );
            } else {
                println!(
                    "No day argument given. Today is day {}, so trying that day...",
                    current_day
                );
            }
            let func = get_day_solver(current_day as u8);
            let time = Instant::now();
            let (p1_curr, p2_curr) = func();
            elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
            p1 = p1_curr;
            p2 = p2_curr;
            if p1 != Solution::from(0) && p2 != Solution::from(0) {
                break;
            }
            current_day -= 1;
        }

        println!("\n=== Day {:02} ===", current_day);
        println!("  · Part 1: {}", p1);
        println!("  · Part 2: {}", p2);
        println!("  · Elapsed: {:.4} ms", elapsed_ms);
        return;
    }

    // Check if -all is given as an argument, and if so, run all days.
    let days: Vec<u8> = if args[1] == "-all" {
        (1..=25).collect()
    } else {
        // Otherwise, parse the arguments as days.
        args[1..]
            .iter()
            .map(|x| {
                x.parse()
                    .unwrap_or_else(|v| panic!("Not a valid day: {}", v))
            })
            .collect()
    };

    let mut runtimes: HashMap<u8, f64> = HashMap::new();

    for day in days {
        let func = get_day_solver(day);

        let time = Instant::now();
        let (p1, p2) = func();
        let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;

        if p1 == Solution::from(0) && p2 == Solution::from(0) {
            continue;
        }

        println!("\n=== Day {:02} ===", day);
        println!("  · Part 1: {}", p1);
        println!("  · Part 2: {}", p2);
        println!("  · Elapsed: {:.4} ms", elapsed_ms);

        runtimes.insert(day, elapsed_ms);
    }

    let total_runtime = runtimes.values().sum::<f64>();
    println!("\nTotal runtime: {:.4} ms", total_runtime);
    println!(
        "Average runtime: {:.4} ms",
        total_runtime / runtimes.len() as f64
    );

    println!("\n=== Sorted Runtimes ===");
    // Print runtimes in order from fastest to slowest
    runtimes
        .iter()
        .sorted_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .for_each(|(day, runtime)| {
            println!("Day {:02}: {:.4} ms", day, runtime);
        });
}

fn get_day_solver(day: u8) -> fn() -> SolutionPair {
    match day {
        1 => day01::solve,
        2 => day02::solve,
        3 => day03::solve,
        4 => day04::solve,
        5 => day05::solve,
        6 => day06::solve,
        7 => day07::solve,
        8 => day08::solve,
        9 => day09::solve,
        10 => day10::solve,
        11 => day11::solve,
        12 => day12::solve,
        13 => day13::solve,
        14 => day14::solve,
        15 => day15::solve,
        16 => day16::solve,
        17 => day17::solve,
        18 => day18::solve,
        19 => day19::solve,
        20 => day20::solve,
        21 => day21::solve,
        22 => day22::solve,
        23 => day23::solve,
        24 => day24::solve,
        25 => day25::solve,
        _ => unimplemented!(),
    }
}
