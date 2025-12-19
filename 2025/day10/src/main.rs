use good_lp::{constraint, default_solver, variable, variables, Solution, SolverModel};
use std::collections::{HashSet, VecDeque};
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
    let mut total_presses = 0;
    
    for machine in input {
        // Parse goal into bitmask: [.##.] -> bits set where '#' appears
        let mut goal: u32 = 0;
        for (i, c) in machine.light_goal[1..machine.light_goal.len()-1].chars().enumerate() {
            if c == '#' {
                goal |= 1 << i;
            }
        }
        
        // Convert buttons to bitmasks
        let button_masks: Vec<u32> = machine.buttons.iter()
            .map(|btn| btn.iter().fold(0u32, |acc, &i| acc | (1 << i)))
            .collect();
        
        // BFS: find minimum button presses to reach goal from state 0
        let mut queue: VecDeque<(u32, u64)> = VecDeque::new(); // (state, presses)
        let mut visited: HashSet<u32> = HashSet::new();
        
        queue.push_back((0, 0)); // Start with all lights off
        visited.insert(0);
        
        while let Some((state, presses)) = queue.pop_front() {
            if state == goal {
                total_presses += presses;
                break;
            }
            
            // Try each button
            for &mask in &button_masks {
                let new_state = state ^ mask; // XOR toggles lights
                if !visited.contains(&new_state) {
                    visited.insert(new_state);
                    queue.push_back((new_state, presses + 1));
                }
            }
        }
    }
    
    total_presses
}

fn part2(input: Vec<Machine>) -> u64 {
    let mut total_presses = 0;
    
    for machine in input {
        let target = &machine.joltage;
        let num_counters = target.len();
        let num_buttons = machine.buttons.len();
        
        // Build coefficient matrix: button_coefs[button][counter] = 1 if button affects counter
        let button_coefs: Vec<Vec<f64>> = machine.buttons.iter()
            .map(|btn| {
                let mut v = vec![0.0; num_counters];
                for &i in btn {
                    if (i as usize) < num_counters {
                        v[i as usize] = 1.0;
                    }
                }
                v
            })
            .collect();
        
        // Create ILP problem
        variables! {
            problem:
                // x[i] = number of times button i is pressed (integer >= 0)
        }
        
        // Create button press variables
        let x: Vec<_> = (0..num_buttons)
            .map(|_| problem.add(variable().integer().min(0)))
            .collect();
        
        // Objective: minimize total presses
        let objective = x.iter().sum::<good_lp::Expression>();
        
        let mut model = problem.minimise(objective).using(default_solver);
        
        // Constraints: for each counter, sum of contributions = target
        for j in 0..num_counters {
            let mut expr = good_lp::Expression::default();
            for i in 0..num_buttons {
                expr += button_coefs[i][j] * x[i];
            }
            model = model.with(constraint!(expr == target[j] as f64));
        }
        
        // Solve
        let solution = model.solve().unwrap();
        
        // Sum up the button presses
        let presses: f64 = x.iter().map(|&xi| solution.value(xi)).sum();
        total_presses += presses.round() as u64;
    }
    
    total_presses
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
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
    assert_eq!(part2(parse_input(raw_input)), 33);
}
