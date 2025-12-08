use utils::structs::stopwatch::{ReportDuration, Stopwatch};

fn main() {
    let mut watch = Stopwatch::new();
    let input = std::fs::read_to_string("2025/day9/input").expect("Failed to read input file");
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

fn parse_input(input: &str) -> Vec<&str> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect()
}

fn part1(input: Vec<&str>) -> u64 {
    0
}

fn part2(input: Vec<&str>) -> u64 {
    0
}

#[test]
fn test1() {
    let raw_input = r"
";
    assert_eq!(part1(parse_input(raw_input)), 0);
}

#[test]
fn test2() {
    let raw_input = r"
";
    assert_eq!(part2(parse_input(raw_input)), 0);
}
