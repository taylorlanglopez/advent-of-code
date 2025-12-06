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
            let lines : Vec<&str> = input.lines().filter(|line| !line.trim().is_empty()).collect();
            let matrix : Vec<Vec<&str>> = lines.iter().map(|line| line.split_whitespace().collect()).collect();
            let mut current_equation: Vec<&str> = Vec::new();
            let mut answer : u64 = 0;
            for col in 0..matrix[0].len() {
                for row in 0..matrix.len() {
                    match matrix[row][col] {
                        "+" => {
                            answer += current_equation.iter().map(|s| s.parse::<u64>().unwrap()).sum::<u64>();
                        }, 
                        "*" => {
                            answer += current_equation.iter().map(|s| s.parse::<u64>().unwrap()).product::<u64>();
                        },
                        _ => current_equation.push(matrix[row][col]),
                    }
                }
                current_equation.clear();
            }
            answer
        },
        ReadDirection::Part2 => {
            let mut lines: Vec<&str> = input.lines().filter(|line| !line.trim().is_empty()).collect();
            let mut operators_line: Vec<char> = lines.pop().unwrap().split_whitespace().map(|s| s.chars().next().unwrap()).collect();

            // Pre-convert to char grids for O(1) access
            let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
            let width = grid.iter().map(|row| row.len()).max().unwrap_or(0);

            let mut problems: u64 = 0;
            let mut current_operator: char = operators_line.pop().unwrap();
            let mut current_equation: Vec<String> = Vec::new();
            let mut current_num: String = String::new();
            for column_number in (0..width).rev() {
                for row in 0..grid.len() {
                    // O(1) access instead of O(n)
                    let ch = grid[row].get(column_number).copied().unwrap_or(' ');
                    if !ch.is_whitespace() {
                        current_num.push(ch);
                    }
                }
                if current_num.is_empty() || column_number == 0 {
                    if column_number == 0 && !current_num.is_empty() {
                        current_equation.push(current_num.clone());
                    }

                    match current_operator {
                        '+' => {
                            let eq_eval = current_equation.iter().map(|s| s.parse::<u64>().unwrap()).sum::<u64>();
                            problems += eq_eval;
                        },
                        '*' => {
                            let eq_eval = current_equation.iter().map(|s| s.parse::<u64>().unwrap()).product::<u64>();
                            problems += eq_eval;
                        },
                        _ => panic!("Unknown operator"),
                    }
                    if let Some(op) = operators_line.pop() {
                        current_operator = op;
                        current_equation.clear();
                    }
                } else {
                    current_equation.push(current_num.clone());
                    current_num.clear();
                }
            }
            problems
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