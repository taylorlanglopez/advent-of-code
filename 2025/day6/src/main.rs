use utils::structs::stopwatch::{ReportDuration, Stopwatch};

#[derive(Clone, Copy, PartialEq)]
enum ReadDirection {
    Part1,  // Part 1: columns L->R, each row has a complete number
    Part2,  // Part 2: columns R->L, digits form numbers vertically
}

fn main() {
    let mut watch = Stopwatch::new();
    watch.start();
    let input = std::fs::read_to_string("2025/day6/input").expect("Failed to read input file");
    println!(
        "1. {} ({})",
        &parse_input(&input, ReadDirection::Part1),
        watch.lap().report()
    );
    println!(
        "2. {} ({})",
        &parse_input(&input, ReadDirection::Part2),
        watch.lap().report()
    );
}

fn parse_input(input: &str, direction: ReadDirection) -> u64 {
    let problems = match direction {
        ReadDirection::Part1 => {
            let mut lines: Vec<&str> = input.lines().filter(|line| !line.is_empty()).collect();
            let mut numbers_columns: Vec<Vec<u64>> = Vec::new();
            let mut operators: Vec<char> = Vec::new();
            0
        },
        ReadDirection::Part2 => {
            let mut lines: Vec<&str> = input.lines().filter(|line| !line.trim().is_empty()).collect();
            let mut operators_line: Vec<char> = lines.pop().unwrap().split_whitespace().map(|s| s.chars().next().unwrap()).collect();

            let mut problems: u64 = 0;
            let mut current_operator: char = operators_line.pop().unwrap();
            for column_number in (0 .. lines[0].len()).rev() {
                let mut current_num: u64 = 0;
                for row in 0..lines.len() {
                    let ch = lines[row].chars().nth(column_number).unwrap();
                    if ch.is_whitespace() {
                        continue;
                    } else {
                        current_num = current_num * 10 + ch.to_digit(10).unwrap() as u64;
                    }
                }
                if current_num == 0 {
                    current_operator = operators_line.pop().unwrap();
                } else {
                    match current_operator {
                        '+' => problems += current_num,
                        '*' => problems *= current_num,
                        _ => panic!("Unknown operator"),
                    }
                }
            }
            0
        },
    };
    problems
}

#[test]
fn test1() {
    let raw_input = r"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +
";
    assert_eq!(parse_input(raw_input, ReadDirection::Part1), 4277556);
}

#[test]
fn test2() {
    let raw_input = r"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
    assert_eq!(parse_input(raw_input, ReadDirection::Part2), 3263827);
}