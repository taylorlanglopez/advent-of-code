use utils::structs::stopwatch::{ReportDuration, Stopwatch};

fn main() {
    let mut watch = Stopwatch::new();
    watch.start();
    let input = std::fs::read_to_string("2025/day4/input").expect("Failed to read input file");
    println!(
        "1. {} ({})",
        part1(&parse_input(input.clone())),
        watch.lap().report()
    );
    println!(
        "2. {} ({})",
        part2(&parse_input(input.clone())),
        watch.lap().report()
    );
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn part1(input: &Vec<Vec<char>>) -> u64 {
    let mut count = 0;
    let rows = input.len();
    let cols = input[0].len();

    for r in 0..rows {
        for c in 0..cols {
            if input[r][c] == '@' {
                let mut adjacent_count = 0;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        let nr = r as isize + dr;
                        let nc = c as isize + dc;
                        if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                            if input[nr as usize][nc as usize] == '@' {
                                adjacent_count += 1;
                            }
                        }
                    }
                }
                if adjacent_count < 4 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part2(input: &Vec<Vec<char>>) -> i64 {
    let mut count = 0;
    let mut input_clone = input.clone();
    let rows = input_clone.len();
    let cols = input_clone[0].len();

    let mut prev_count: i64 = -1;
    while prev_count != count {
        prev_count = count;
        for r in 0..rows {
            for c in 0..cols {
                if input_clone[r][c] == '@' {
                    let mut adjacent_count = 0;
                    for dr in -1..=1 {
                        for dc in -1..=1 {
                            if dr == 0 && dc == 0 {
                                continue;
                            }
                            let nr = r as isize + dr;
                            let nc = c as isize + dc;
                            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                                if input_clone[nr as usize][nc as usize] == '@' {
                                    adjacent_count += 1;
                                }
                            }
                        }
                    }
                    if adjacent_count < 4 {
                        count += 1;
                        input_clone[r][c] = '.';                    }
                }
            }
        }
    }

    count
}

#[test]
fn test1() {
    let input = "..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.";
    let parsed_input = parse_input(input.to_string());
    assert_eq!(part1(&parsed_input), 13);
}

#[test]
fn test2() {
    let input = "..@@.@@@@.
                       @@@.@.@.@@
                       @@@@@.@.@@
                       @.@@@@..@.
                       @@.@@@@.@@
                       .@@@@@@@.@
                       .@.@.@.@@@
                       @.@@@.@@@@
                       .@@@@@@@@.
                       @.@.@@@.@.";
    let parsed_input = parse_input(input.to_string());
    assert_eq!(part2(&parsed_input), 43);
}