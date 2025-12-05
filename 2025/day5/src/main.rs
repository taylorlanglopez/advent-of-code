use utils::structs::stopwatch::{ReportDuration, Stopwatch};
use regex::Regex;

struct Interval {
    start: u64,
    end: u64,
}

struct Union {
    intervals: Vec<Interval>,
}

impl Union {
    fn new() -> Self {
        Union { intervals: vec![] }
    }

    fn add(&mut self, mut new_interval: Interval) {
        let mut i = 0;
        while i < self.intervals.len() {
            let current = &self.intervals[i];
            if new_interval.end < current.start {
                break;
            } else if new_interval.start > current.end {
                i += 1;
                continue;
            } else {
                new_interval.start = new_interval.start.min(current.start);
                new_interval.end = new_interval.end.max(current.end);
                self.intervals.remove(i);
            }
        }
        self.intervals.insert(i, new_interval);
    }

    fn total_covered(&self) -> u64 {
        self.intervals.iter().map(|interval| interval.end - interval.start + 1).sum()
    }

    fn count_covered(&self, queries: &Vec<u64>) -> u64 {
        let mut count = 0;
        for &query in queries {
            for interval in &self.intervals {
                if query >= interval.start && query <= interval.end {
                    count += 1;
                    break;
                }
            }
        }
        count
    }
}

fn main() {
    let mut watch = Stopwatch::new();
    watch.start();
    let input = std::fs::read_to_string("2025/day5/input").expect("Failed to read input file");
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

fn parse_input(input: String) -> (Vec<Interval>, Vec<u64>) {
    let re = Regex::new(r"\n\s*\n").unwrap();
    let input = input.trim();
    let parts: Vec<&str> = re.split(input).collect();
    
    let intervals = parts[0]
        .lines()
        .map(|line| {
            let line = line.trim();
            let mut nums = line.split('-').map(|s| s.parse::<u64>().unwrap());
            Interval {
                start: nums.next().unwrap(),
                end: nums.next().unwrap(),
            }
        })
        .collect();

    let queries = parts[1]
        .lines()
        .map(|line| line.trim().parse::<u64>().unwrap())
        .collect();
    
    (intervals, queries)
}

fn part1(input: &(Vec<Interval>, Vec<u64>)) -> usize {
    let (intervals, queries) = input;
    let mut union : Union = Union::new();

    for interval in intervals {
        union.add(Interval {
            start: interval.start,
            end: interval.end,
        });
    }

    union.count_covered(queries) as usize
}

fn part2(input: &(Vec<Interval>, Vec<u64>)) -> usize {
    let (intervals, _) = input;
    let mut union : Union = Union::new();

    for interval in intervals {
        union.add(Interval {
            start: interval.start,
            end: interval.end,
        });
    }

    union.total_covered() as usize
}

#[test]
fn test1() {
    let input = r"3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32
    ";
    assert_eq!(part1(&parse_input(input.to_string())), 3);
}

#[test]
fn test2() {
    let input = r"3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32
    ";
    assert_eq!(part2(&parse_input(input.to_string())), 14);
}