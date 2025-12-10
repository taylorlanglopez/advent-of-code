use utils::structs::stopwatch::{ReportDuration, Stopwatch};

type Pair = (u64, u64);

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

fn parse_input(input: &str) -> Vec<Pair> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split(',').map(|s| s.trim().parse::<u64>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect()
}

fn calculate_area(first : (u64, u64), second: (u64, u64)) -> u64 {
    let third : (u64, u64) = (first.0, second.1);
    let fourth : (u64, u64) = (second.0, first.1);

    if first.0 == third.0 {
        // first and third are the same row
        let x = (first.1 as i64 - third.1 as i64).abs() + 1; // col distance
        let y: i64 = (first.0 as i64 - fourth.0 as i64).abs() + 1; 
        return (x * y) as u64;
    }

    if first.0 == fourth.0 {
        // first and fourth are the same row
        let x: i64 = (first.1 as i64 - fourth.1 as i64).abs() + 1; // col distance
        let y: i64 = (first.0 as i64 - third.0 as i64).abs() + 1;
        return (x * y) as u64;
    }

    0
}

fn part1(input: Vec<Pair>) -> u64 {
    let mut largest_area = 0;
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j { continue; }
            let current_area = calculate_area(input[i], input[j]); 
            if current_area >= largest_area {
                largest_area = current_area;
            }
        }
    }
    largest_area
}

fn part2(input: Vec<Pair>) -> u64 {
    let n = input.len();
    if n < 2 {
        return 0;
    }

    // Build polygon from input (vertices in order)
    let polygon: Vec<(u64, u64)> = input.clone();
    
    // Function to check if a point is on the polygon boundary (green tile)
    let is_on_edge = |px: u64, py: u64| -> bool {
        for i in 0..n {
            let (x1, y1) = polygon[i];
            let (x2, y2) = polygon[(i + 1) % n];
            
            if y1 == y2 && py == y1 {
                let (xmin, xmax) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                if px >= xmin && px <= xmax {
                    return true;
                }
            }
            if x1 == x2 && px == x1 {
                let (ymin, ymax) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
                if py >= ymin && py <= ymax {
                    return true;
                }
            }
        }
        false
    };
    
    // Function to check if a point is inside the polygon using ray casting
    let is_inside_polygon = |px: u64, py: u64| -> bool {
        if is_on_edge(px, py) {
            return true;
        }
        
        let mut crossings = 0;
        for i in 0..n {
            let (x1, y1) = polygon[i];
            let (x2, y2) = polygon[(i + 1) % n];
            
            if x1 != x2 {
                continue;
            }
            
            let edge_x = x1;
            let (ymin, ymax) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            
            if edge_x > px && py > ymin && py < ymax {
                crossings += 1;
            }
        }
        
        crossings % 2 == 1
    };

    // Function to check if rectangle is valid (entirely inside polygon)
    // For rectilinear polygons: valid if all 4 corners are inside AND no polygon edge
    // passes through the interior of the rectangle
    let is_valid_rectangle = |x1: u64, y1: u64, x2: u64, y2: u64| -> bool {
        let (xmin, xmax) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (ymin, ymax) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        
        // Check all 4 corners are inside polygon
        if !is_inside_polygon(xmin, ymin) { return false; }
        if !is_inside_polygon(xmin, ymax) { return false; }
        if !is_inside_polygon(xmax, ymin) { return false; }
        if !is_inside_polygon(xmax, ymax) { return false; }
        
        // Check no polygon edge passes through interior of rectangle
        for i in 0..n {
            let (ex1, ey1) = polygon[i];
            let (ex2, ey2) = polygon[(i + 1) % n];
            
            if ex1 == ex2 {
                // Vertical edge at x = ex1
                let edge_x = ex1;
                let (edge_ymin, edge_ymax) = if ey1 < ey2 { (ey1, ey2) } else { (ey2, ey1) };
                
                // Does this edge pass through interior? (not just touch boundary)
                // Edge must be strictly inside x range, and overlap y range
                if edge_x > xmin && edge_x < xmax {
                    // Check if edge overlaps with rectangle's y range
                    if edge_ymin < ymax && edge_ymax > ymin {
                        return false; // Edge crosses interior
                    }
                }
            } else if ey1 == ey2 {
                // Horizontal edge at y = ey1
                let edge_y = ey1;
                let (edge_xmin, edge_xmax) = if ex1 < ex2 { (ex1, ex2) } else { (ex2, ex1) };
                
                // Does this edge pass through interior?
                if edge_y > ymin && edge_y < ymax {
                    // Check if edge overlaps with rectangle's x range
                    if edge_xmin < xmax && edge_xmax > xmin {
                        return false; // Edge crosses interior
                    }
                }
            }
        }
        
        true
    };
    
    // Check all pairs of red tiles
    let mut largest_area: u64 = 0;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let (x1, y1) = input[i];
            let (x2, y2) = input[j];
            
            if is_valid_rectangle(x1, y1, x2, y2) {
                let width = (x1 as i64 - x2 as i64).abs() as u64 + 1;
                let height = (y1 as i64 - y2 as i64).abs() as u64 + 1;
                let area = width * height;
                if area > largest_area {
                    largest_area = area;
                }
            }
        }
    }
    
    largest_area
}
#[test]
fn test1() {
    let raw_input = r"
    7,1
    11,1
    11,7
    9,7
    9,5
    2,5
    2,3
    7,3
";
    assert_eq!(part1(parse_input(raw_input)), 50);
}

#[test]
fn test2() {
    let raw_input = r"
    7,1
    11,1
    11,7
    9,7
    9,5
    2,5
    2,3
    7,3
";
    assert_eq!(part2(parse_input(raw_input)), 24);
}
