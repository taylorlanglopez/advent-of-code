use std::collections::HashMap;
use utils::structs::stopwatch::{ReportDuration, Stopwatch};

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

// Union-Find with path compression and union by rank
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    num_sets: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            num_sets: n,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let pi = self.find(i);
        let pj = self.find(j);
        if pi == pj {
            return false;
        }
        // Union by rank
        if self.rank[pi] < self.rank[pj] {
            self.parent[pi] = pj;
        } else if self.rank[pi] > self.rank[pj] {
            self.parent[pj] = pi;
        } else {
            self.parent[pj] = pi;
            self.rank[pi] += 1;
        }
        self.num_sets -= 1;
        true
    }
}

fn main() {
    let mut watch = Stopwatch::new();
    let input = std::fs::read_to_string("2025/day8/input").expect("Failed to read input file");
    let points = parse_input(&input);
    let edges = build_sorted_edges(&points);
    watch.start();
    println!(
        "1. {} ({})",
        part1(&points, &edges),
        watch.lap().report()
    );
    println!(
        "2. {} ({})",
        part2(&points, &edges),
        watch.lap().report()
    );
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let coords: Vec<i64> = line
                .split(',')
                .map(|num| num.trim().parse().unwrap())
                .collect();
            Point { x: coords[0], y: coords[1], z: coords[2] }
        })
        .collect()
}

fn distance_squared(p1: &Point, p2: &Point) -> i64 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    let dz = p1.z - p2.z;
    dx * dx + dy * dy + dz * dz
}

fn build_sorted_edges(points: &[Point]) -> Vec<(i64, usize, usize)> {
    let n = points.len();
    let mut edges: Vec<(i64, usize, usize)> = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((distance_squared(&points[i], &points[j]), i, j));
        }
    }
    edges.sort_unstable_by_key(|e| e.0);
    edges
}

fn solve(points: &[Point], edges: &[(i64, usize, usize)], num_connections: usize) -> u64 {
    let n = points.len();
    if n <= 1 {
        return 1;
    }

    let mut uf = UnionFind::new(n);
    
    let mut connections_made = 0;
    for &(_, i, j) in edges {
        if connections_made >= num_connections {
            break;
        }
        uf.union(i, j);
        connections_made += 1;
    }

    let mut sizes: HashMap<usize, u64> = HashMap::new();
    for i in 0..n {
        let root = uf.find(i);
        *sizes.entry(root).or_insert(0) += 1;
    }

    let mut size_vec: Vec<u64> = sizes.into_values().collect();
    size_vec.sort_unstable_by(|a, b| b.cmp(a));
    size_vec.iter().take(3).product()
}

fn part1(points: &[Point], edges: &[(i64, usize, usize)]) -> u64 {
    solve(points, edges, 1000)
}

fn part2(points: &[Point], edges: &[(i64, usize, usize)]) -> u64 {
    let n = points.len();
    if n <= 1 {
        return 0;
    }

    let mut uf = UnionFind::new(n);
    let mut last_merged_pair = (0, 0);

    for &(_, i, j) in edges {
        if uf.num_sets == 1 {
            break;
        }
        if uf.union(i, j) {
            last_merged_pair = (i, j);
        }
    }

    (points[last_merged_pair.0].x * points[last_merged_pair.1].x) as u64
}

#[test]
fn test1() {
    let raw_input = r"
    162,817,812
    57,618,57
    906,360,560
    592,479,940
    352,342,300
    466,668,158
    542,29,236
    431,825,988
    739,650,466
    52,470,668
    216,146,977
    819,987,18
    117,168,530
    805,96,715
    346,949,466
    970,615,88
    941,993,340
    862,61,35
    984,92,344
    425,690,689
    ";
    let points = parse_input(raw_input);
    let edges = build_sorted_edges(&points);
    assert_eq!(solve(&points, &edges, 10), 40);
}

#[test]
fn test2() {
    let raw_input = r"
    162,817,812
    57,618,57
    906,360,560
    592,479,940
    352,342,300
    466,668,158
    542,29,236
    431,825,988
    739,650,466
    52,470,668
    216,146,977
    819,987,18
    117,168,530
    805,96,715
    346,949,466
    970,615,88
    941,993,340
    862,61,35
    984,92,344
    425,690,689
    ";
    let points = parse_input(raw_input);
    let edges = build_sorted_edges(&points);
    assert_eq!(part2(&points, &edges), 25272);
}
