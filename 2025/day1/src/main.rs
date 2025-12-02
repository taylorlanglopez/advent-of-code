use utils::structs::stopwatch::{ReportDuration, Stopwatch};

fn main() -> Result <(), std::io::Error> {
    let mut watch = Stopwatch::new();
    let input = std::fs::read_to_string("input")?;
    println!("1. {} ({})", part1(&parse_input(input.clone())), watch.lap().report());
    println!("2. {} ({})", part2(&parse_input(input.clone())), watch.lap().report());
    Ok(())
}

type Input = Vec<i64>;

fn part1(input: &Input) -> usize {
    input.iter().scan(50i64, |state, &n| {
        *state = (*state + n).rem_euclid(100);
        Some(*state)
    }).filter(|&n| n == 0)
    .count()
}

fn part2(input: &Input) -> usize {
    let mut dial = 50;
    let mut clicks = 0;
    for &n in input {
        dial += n;
        if dial <= 0 && n != dial {
            clicks += 1;
        }
        clicks += dial.abs() / 100;
        dial = dial.rem_euclid(100);
    }
    clicks as usize
}

fn parse_input(input: String) -> Input {
    input
        .lines()
        .map(|line| {
            let (dir, n) = line.split_at(1);
            n.parse::<i64>().unwrap() * if dir == "L" { -1 } else { 1 }
        })
        .collect()
}

#[test]
fn test1() {
    let input = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
    assert_eq!(3, part1(&parse_input(input.to_string())));
}

#[test]
fn test2() {
    let input = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
    assert_eq!(6, part2(&parse_input(input.to_string())));
}