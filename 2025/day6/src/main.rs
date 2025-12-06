use utils::structs::stopwatch::{ReportDuration, Stopwatch};

#[derive(Clone, Copy, PartialEq)]
enum ReadDirection {
    LeftToRight,  // Part 1: columns L->R, each row has a complete number
    RightToLeft,  // Part 2: columns R->L, digits form numbers vertically
}

fn main() {
    let mut watch = Stopwatch::new();
    watch.start();
    let input = std::fs::read_to_string("2025/day6/input").expect("Failed to read input file");
    println!(
        "1. {} ({})",
        solve_problems_set(&parse_input(&input, ReadDirection::LeftToRight)),
        watch.lap().report()
    );
    println!(
        "2. {} ({})",
        solve_problems_set(&parse_input(&input, ReadDirection::RightToLeft)),
        watch.lap().report()
    );
}

fn parse_input(input: &str, direction: ReadDirection) -> Vec<(Vec<u64>, char)> {
    let mut lines: Vec<&str> = input.lines().collect();
    while !lines.is_empty() && lines.first().unwrap().trim().is_empty() {
        lines.remove(0);
    }
    while !lines.is_empty() && lines.last().unwrap().trim().is_empty() {
        lines.pop();
    }

    if lines.is_empty() {
        return Vec::new();
    }

    // Build character grid, pad lines to equal width
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let rows = lines.len();
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|l| {
            let mut chars: Vec<char> = l.chars().collect();
            chars.resize(width, ' ');
            chars
        })
        .collect();

    let mut problems: Vec<(Vec<u64>, char)> = Vec::new();
    
    let mut operator_cols: Vec<(usize, char)> = Vec::new();
    for c in 0..width {
        let last_row_char = grid[rows - 1][c];
        if last_row_char == '+' || last_row_char == '*' {
            operator_cols.push((c, last_row_char));
        }
    }

    match direction {
        ReadDirection::LeftToRight => operator_cols.sort_by_key(|(c, _)| *c),
        ReadDirection::RightToLeft => operator_cols.sort_by_key(|(c, _)| std::cmp::Reverse(*c)),
    }

    let is_separator = |c: usize| -> bool {
        (0..rows).all(|r| grid[r][c] == ' ')
    };

    for (op_col, operator) in operator_cols {
        let mut start_col = op_col;
        while start_col > 0 && !is_separator(start_col - 1) {
            start_col -= 1;
        }
        
        let mut end_col = op_col;
        while end_col + 1 < width && !is_separator(end_col + 1) {
            end_col += 1;
        }

        let numbers = match direction {
            ReadDirection::LeftToRight => {
                let mut nums: Vec<u64> = Vec::new();
                for r in 0..(rows - 1) {
                    let row_slice: String = grid[r][start_col..=end_col].iter().collect();
                    let trimmed = row_slice.trim();
                    if !trimmed.is_empty() {
                        if let Ok(n) = trimmed.parse::<u64>() {
                            nums.push(n);
                        }
                    }
                }
                nums
            }
            ReadDirection::RightToLeft => {
                let mut nums: Vec<u64> = Vec::new();
                let col_range: Vec<usize> = (start_col..=end_col).rev().collect();
                
                for c in col_range {
                    let mut num_str = String::new();
                    for r in 0..(rows - 1) {
                        let ch = grid[r][c];
                        if ch.is_ascii_digit() {
                            num_str.push(ch);
                        }
                    }
                    if !num_str.is_empty() {
                        if let Ok(n) = num_str.parse::<u64>() {
                            nums.push(n);
                        }
                    }
                }
                nums
            }
        };

        problems.push((numbers, operator));
    }

    problems
}

fn solve_problems_set(problems: &Vec<(Vec<u64>, char)>) -> u64 {
    let mut result: u64 = 0;

    for (numbers, operator) in problems {
        match operator {
            '*' => {
                let product: u64 = numbers.iter().product();
                result += product;
            },
            '+' => {
                let sum: u64 = numbers.iter().sum();
                result += sum;
            },
            _ => panic!("Unknown operator"),
        }
    }

    result
}

#[test]
fn test1() {
    let raw_input = r"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +
";
    assert_eq!(solve_problems_set(&parse_input(raw_input, ReadDirection::LeftToRight)), 4277556);
}

#[test]
fn test2() {
    let raw_input = r"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
    assert_eq!(solve_problems_set(&parse_input(raw_input, ReadDirection::RightToLeft)), 3263827);
}