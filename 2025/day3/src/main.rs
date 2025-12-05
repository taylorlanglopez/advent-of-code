use utils::structs::stopwatch::{ReportDuration, Stopwatch};

fn main() {
    let mut watch = Stopwatch::new();
    watch.start();
    let input = std::fs::read_to_string("2025/day3/input").expect("Failed to read input file");
    println!("1. {} ({})", part1(&parse_input(input.clone())), watch.lap().report());
    println!("2. {} ({})", part2(&parse_input(input.clone()), 12), watch.lap().report());
}

fn parse_input(input: String) -> Vec<String> {
    input
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn part1(input: &Vec<String>) -> u64 {
    input.iter()
        .map(|n| {
            let digits : Vec<u32> = n
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect();
            let mut largest_num: u32 = 0;
            for i in 0..digits.len() {
                for j in (i+1)..digits.len() {
                    if (digits[i] * 10 + digits[j]) > largest_num{
                        largest_num = digits[i] * 10 + digits[j];
                    }
                }
            }
            largest_num as u64
        }).sum()  
}

fn part2(input: &Vec<String>, num_batteries: usize) -> u64 {
    input.iter()
        .map(|n| {
            let digits : Vec<u32> = n
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect();
            if digits.len() < num_batteries {
                return 0;
            }
            let mut turned_on : Vec<u32> = digits[0..num_batteries].to_vec();
            for window in digits.windows(num_batteries).skip(1) {
                for i in 0..num_batteries {
                    if window[i] > turned_on[i] {
                        turned_on.truncate(i);
                        turned_on.append(&mut window[i..].to_vec());
                        break;
                    }
                }
            }
            turned_on.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
        }).sum()
}

#[test]
fn test1()
{
    let input = r"
    987654321111111
    811111111111119
    234234234234278
    818181911112111
    ";
    assert_eq!(357, part1(&parse_input(input.to_string())));
}

#[test]
fn test2()
{
    let input = r"
    987654321111111
    811111111111119
    234234234234278
    818181911112111
    ";
    assert_eq!(3121910778619, part2(&parse_input(input.to_string()), 12));
}