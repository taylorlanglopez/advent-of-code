use std::fmt::Debug;
use utils::structs::stopwatch::{ReportDuration, Stopwatch};

struct Machine {
    light_goal : String,
    buttons : Vec<Vec<u64>>,
    joltage : Vec<u64>
}

impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n  Machine {{ light_goal: {}, buttons: {:?}, joltage: {:?} }}",
            self.light_goal, self.buttons, self.joltage
        )
    }
}

fn main() {
    let mut watch = Stopwatch::new();
    let input = std::fs::read_to_string("2025/day10/input").expect("Failed to read input file");
    watch.start();
    println!(
        "1. {} ({})",
        part1(parse_input(&input)),
        watch.lap().report()
    );
    println!(
        "2. {} ({})",
        part2(parse_input(&input)),
        watch.lap().report()
    );
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let lights = parts.next().unwrap().to_string();
            let mut buttons: Vec<Vec<u64>> = Vec::new();
            let mut joltage: Vec<u64> = Vec::new();
            for part in parts {
                if part.starts_with('(') {
                    let inner_parts = part[1..part.len() - 1].split(',');
                    buttons.push(inner_parts.map(|p| p.parse().unwrap()).collect());
                } else if part.starts_with('{') {
                    let inner_parts = part[1..part.len() - 1].split(',');
                    joltage = inner_parts.map(|p| p.parse().unwrap()).collect();
                }
            }
            Machine { light_goal: lights, buttons, joltage }
        })
        .collect()
}

fn part1(input: Vec<Machine>) -> u64 {
    println!("{:?}", input);
    0
}

fn part2(input: Vec<Machine>) -> u64 {
    println!("{:?}", input);
    0
}

#[test]
fn test1() {
    let raw_input = r"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
    assert_eq!(part1(parse_input(raw_input)), 7);
}

#[test]
fn test2() {
    let raw_input = r"
";
    assert_eq!(part2(parse_input(raw_input)), 0);
}
