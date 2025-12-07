use utils::structs::stopwatch::{ReportDuration, Stopwatch};

fn main() {
    let mut watch = Stopwatch::new();
    watch.start();
    let input = std::fs::read_to_string("2025/day7/input").expect("Failed to read input file");
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

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    lines.iter().map(|f| f.chars().collect()).collect()
}

fn part1(mut tachyon_map: Vec<Vec<char>>) -> u64 {
    let mut beam_splits: u64 = 0;
    let height = tachyon_map.len();

    let start = tachyon_map[0]
        .iter()
        .position(|&c| c == 'S')
        .expect("Couldn't find tachyon start") as u64;
    let mut beam_stack: Vec<(u64, u64)> = Vec::new();
    beam_stack.push((0, start));
    while !beam_stack.is_empty() {
        let pos = beam_stack.pop().unwrap();
        if pos.0 + 1 >= height as u64 {
            continue;
        }
        let peek = tachyon_map[(pos.0 + 1) as usize][pos.1 as usize];
        match peek {
            '^' => {
                beam_splits += 1;
                tachyon_map[(pos.0 + 1) as usize][pos.1 as usize] = '|';
                beam_stack.push((pos.0 + 1, pos.1 + 1));
                beam_stack.push((pos.0 + 1, pos.1 - 1));
            }
            '.' => {
                beam_stack.push((pos.0 + 1, pos.1));
            }
            _ => {
                continue;
            }
        }
    }

    beam_splits
}

fn part2(tachyon_map: Vec<Vec<char>>) -> u64 {
    let height = tachyon_map.len();
    let mut total_paths: Vec<u64> = vec![0; tachyon_map[0].len()];
    for row in 0..height {
        for (index, space) in tachyon_map[row].iter().enumerate() {
            match space {
                'S' => total_paths[index] = 1,
                '^' => {
                    total_paths[index - 1] += total_paths[index];
                    total_paths[index + 1] += total_paths[index];

                    total_paths[index] = 0;
                }
                _ => (),
            }
        }
    }

    total_paths.iter().sum()
}

#[test]
fn test1() {
    let input = r"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
    assert_eq!(part1(parse_input(input)), 21);
}

#[test]
fn test2() {
    let input = r"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
    assert_eq!(part2(parse_input(input)), 40);
}
