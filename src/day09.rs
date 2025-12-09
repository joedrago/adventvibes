use std::collections::HashMap;
use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/day09.txt").expect("Could not read input file");

    let part1 = solve_part1(&input);
    println!("Part 1: {}", part1);

    let part2 = solve_part2(&input);
    println!("Part 2: {}", part2);
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<i64> = line
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            (parts[0], parts[1])
        })
        .collect()
}

fn solve_part1(input: &str) -> i64 {
    let tiles = parse_input(input);
    let n = tiles.len();

    let mut max_area = 0;

    // Check all pairs of tiles as opposite corners
    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];

            // Rectangle area with these as opposite corners
            // Add 1 to each dimension because the rectangle includes the corner tiles
            let width = (x2 - x1).abs() + 1;
            let height = (y2 - y1).abs() + 1;
            let area = width * height;

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

fn solve_part2(input: &str) -> i64 {
    let tiles = parse_input(input);
    let n = tiles.len();

    if n == 0 {
        return 0;
    }

    // Build polygon edges
    let mut edges: Vec<((i64, i64), (i64, i64))> = Vec::new();
    for i in 0..n {
        let p1 = tiles[i];
        let p2 = tiles[(i + 1) % n];
        edges.push((p1, p2));
    }

    // Coordinate compression: collect all unique x and y coordinates
    let mut xs: Vec<i64> = tiles.iter().map(|t| t.0).collect();
    let mut ys: Vec<i64> = tiles.iter().map(|t| t.1).collect();
    xs.sort();
    xs.dedup();
    ys.sort();
    ys.dedup();

    let x_to_idx: HashMap<i64, usize> = xs.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    let y_to_idx: HashMap<i64, usize> = ys.iter().enumerate().map(|(i, &y)| (y, i)).collect();

    // For cells between coordinates, check if inside polygon
    // Cell (i, j) represents the region between xs[i]..xs[i+1] and ys[j]..ys[j+1]
    let num_x_cells = if xs.len() > 1 { xs.len() - 1 } else { 0 };
    let num_y_cells = if ys.len() > 1 { ys.len() - 1 } else { 0 };

    let mut cell_valid = vec![vec![false; num_y_cells]; num_x_cells];

    for i in 0..num_x_cells {
        for j in 0..num_y_cells {
            // Check center of cell
            let cx = (xs[i] + xs[i + 1]) / 2;
            let cy = (ys[j] + ys[j + 1]) / 2;
            cell_valid[i][j] = is_inside_polygon((cx, cy), &edges);
        }
    }

    // Build prefix sum for fast queries
    let mut prefix = vec![vec![0i64; num_y_cells + 1]; num_x_cells + 1];
    for i in 0..num_x_cells {
        for j in 0..num_y_cells {
            let inv = if cell_valid[i][j] { 0 } else { 1 };
            prefix[i + 1][j + 1] = inv + prefix[i][j + 1] + prefix[i + 1][j] - prefix[i][j];
        }
    }

    let count_invalid_cells = |x1: usize, y1: usize, x2: usize, y2: usize| -> i64 {
        if x2 < x1 || y2 < y1 {
            return 0;
        }
        prefix[x2 + 1][y2 + 1] - prefix[x1][y2 + 1] - prefix[x2 + 1][y1] + prefix[x1][y1]
    };

    // Find largest rectangle with red corners where all interior cells are valid
    let mut max_area = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];

            let xi1 = *x_to_idx.get(&x1.min(x2)).unwrap();
            let xi2 = *x_to_idx.get(&x1.max(x2)).unwrap();
            let yi1 = *y_to_idx.get(&y1.min(y2)).unwrap();
            let yi2 = *y_to_idx.get(&y1.max(y2)).unwrap();

            // Check cells from xi1 to xi2-1 and yi1 to yi2-1
            // These are the interior cells of the rectangle
            let valid = if xi2 > xi1 && yi2 > yi1 {
                count_invalid_cells(xi1, yi1, xi2 - 1, yi2 - 1) == 0
            } else {
                // Degenerate rectangle (line or point) - always valid if corners are red
                true
            };

            if valid {
                let area = (x1.max(x2) - x1.min(x2) + 1) * (y1.max(y2) - y1.min(y2) + 1);
                if area > max_area {
                    max_area = area;
                }
            }
        }
    }

    max_area
}

// Ray casting to determine if point is inside polygon (for axis-aligned edges)
fn is_inside_polygon(point: (i64, i64), edges: &[((i64, i64), (i64, i64))]) -> bool {
    let (px, py) = point;
    let mut crossings = 0;

    for &((x1, y1), (x2, y2)) in edges {
        if x1 == x2 {
            // Vertical edge at x = x1
            let x = x1;
            let (min_y, max_y) = (y1.min(y2), y1.max(y2));

            // Ray goes from (px, py) to (+inf, py)
            // Crosses this edge if edge is to the right and py is in [min_y, max_y)
            if x > px && py >= min_y && py < max_y {
                crossings += 1;
            }
        }
        // Skip horizontal edges for ray casting
    }

    crossings % 2 == 1
}
