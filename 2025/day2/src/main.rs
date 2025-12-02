use utils::structs::stopwatch::{ReportDuration, Stopwatch};

fn main() {
    let mut watch = Stopwatch::new();
    watch.start();
    let input = std::fs::read_to_string("input").expect("Failed to read input file");
    println!("1. {} ({})", part1(&parse_input(input.clone())), watch.lap().report());
    println!("2. {} ({})", part2(&parse_input(input.clone())), watch.lap().report());
}

fn parse_input(input: String) -> Vec<(u64, u64)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            parts
                .iter()
                .map(|part| {
                    let range: Vec<u64> = part
                        .split('-')
                        .map(|num| num.parse().expect("Invalid number"))
                        .collect();
                    (range[0], range[1])
                })
                .collect::<Vec<(u64, u64)>>()
        })
        .flatten()
        .collect()
}

fn part1(input: &Vec<(u64, u64)>) -> u64 {
    let mut count = 0;
    for &(start, end) in input {
        let numbers: Vec<String> = (start..=end)
            .map(|n| n.to_string())
            .filter(|s| s.len().rem_euclid(2) == 0)
            .collect::<Vec<String>>();
        let sum = numbers
            .iter()
            .map(|s| {
                let left = s
                    .chars()
                    .take(s.len() / 2)
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
                let right = s
                    .chars()
                    .skip(s.len() / 2)
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
                if left == right {
                    s.parse::<u64>().unwrap()
                } else {
                    0
                }
            })
            .sum::<u64>();
        count += sum;
    }
    count
}

fn part2(input: &Vec<(u64, u64)>) -> u64 {
    let mut count = 0;
    for &(start, end) in input {
        let numbers: Vec<String> = (start..=end)
            .map(|n| n.to_string())
            .collect::<Vec<String>>();
        for s in numbers {
            let mut window_size = if s.len().rem_euclid(2) == 0 {
                s.len() / 2
            } else {
                (s.len() + 1) / 2
            };
            while window_size > 0 {
                let chunks: Vec<&str> = s
                .as_bytes()
                .chunks(window_size)
                .map(|chunk| std::str::from_utf8(chunk).unwrap())
                .collect();
                
                window_size -= 1;
                
                if chunks.len() < 2 {
                    continue;
                }

                if chunks.iter().skip(1).all(|&c| c == chunks[0]) {
                    count += s.parse::<u64>().unwrap();
                    break;
                }
            }
        }
    }
    count
}

#[test]
fn test1() {
    let input = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862";
    assert_eq!(1227775554, part1(&parse_input(input.to_string())));
}

#[test]
fn test2() {
    let input = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    assert_eq!(4174379265, part2(&parse_input(input.to_string())));
}
