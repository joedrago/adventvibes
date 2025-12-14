use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/day12.txt").expect("Could not read input file");

    let part1 = solve_part1(&input);
    println!("Part 1: {}", part1);

    let part2 = solve_part2(&input);
    println!("Part 2: {}", part2);
}

#[derive(Clone, Debug)]
struct Shape {
    cells: Vec<(i32, i32)>, // relative positions of # cells
}

impl Shape {
    fn new(grid: &[String]) -> Self {
        let mut cells = Vec::new();
        for (r, row) in grid.iter().enumerate() {
            for (c, ch) in row.chars().enumerate() {
                if ch == '#' {
                    cells.push((r as i32, c as i32));
                }
            }
        }
        Shape { cells }
    }

    fn rotations_and_flips(&self) -> Vec<Shape> {
        let mut variations = Vec::new();
        let mut current = self.clone();

        // 4 rotations
        for _ in 0..4 {
            variations.push(current.clone());
            variations.push(current.flipped());
            current = current.rotated();
        }

        // Normalize and deduplicate
        let mut unique = Vec::new();
        for var in variations {
            let normalized = var.normalized();
            if !unique.iter().any(|s: &Shape| s.cells == normalized.cells) {
                unique.push(normalized);
            }
        }
        unique
    }

    fn rotated(&self) -> Shape {
        // Rotate 90 degrees clockwise: (r, c) -> (c, -r)
        let cells: Vec<(i32, i32)> = self.cells.iter().map(|(r, c)| (*c, -*r)).collect();
        Shape { cells }
    }

    fn flipped(&self) -> Shape {
        // Flip horizontally: (r, c) -> (r, -c)
        let cells: Vec<(i32, i32)> = self.cells.iter().map(|(r, c)| (*r, -*c)).collect();
        Shape { cells }
    }

    fn normalized(&self) -> Shape {
        if self.cells.is_empty() {
            return self.clone();
        }
        let min_r = self.cells.iter().map(|(r, _)| r).min().unwrap();
        let min_c = self.cells.iter().map(|(_, c)| c).min().unwrap();
        let cells: Vec<(i32, i32)> = self
            .cells
            .iter()
            .map(|(r, c)| (r - min_r, c - min_c))
            .collect();
        Shape { cells }
    }
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<(usize, usize, Vec<usize>)>) {
    let mut shapes = Vec::new();
    let mut regions = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;

    // Parse shapes - continue until we hit a line with "x" in it
    while i < lines.len() {
        let line = lines[i];

        // Check if this is a region line (contains "x" like "47x47:")
        if line.contains('x') && line.contains(':') {
            break;
        }

        if line.ends_with(':') {
            // Shape header
            i += 1;
            let mut shape_lines = Vec::new();
            while i < lines.len() && !lines[i].is_empty() && !lines[i].ends_with(':') {
                shape_lines.push(lines[i].to_string());
                i += 1;
            }
            shapes.push(Shape::new(&shape_lines));
        } else {
            i += 1;
        }
    }

    // Parse regions
    while i < lines.len() {
        let line = lines[i];
        if line.is_empty() {
            i += 1;
            continue;
        }

        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            let dims: Vec<&str> = parts[0].split('x').collect();
            if dims.len() == 2 {
                if let (Ok(width), Ok(height)) = (dims[0].parse::<usize>(), dims[1].parse::<usize>()) {
                    let counts: Vec<usize> = parts[1]
                        .split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    regions.push((width, height, counts));
                }
            }
        }
        i += 1;
    }

    (shapes, regions)
}

fn can_fit_region(
    shapes: &[Shape],
    width: usize,
    height: usize,
    required: &[usize],
) -> bool {
    let mut grid = vec![vec![false; width]; height];
    let mut shape_list = Vec::new();

    // Build list of shapes to place
    for (idx, &count) in required.iter().enumerate() {
        for _ in 0..count {
            shape_list.push(idx);
        }
    }

    // Quick size check: count total cells needed
    let mut total_cells = 0;
    for (idx, &count) in required.iter().enumerate() {
        total_cells += shapes[idx].cells.len() * count;
    }
    let grid_size = width * height;
    if total_cells > grid_size {
        return false; // Can't possibly fit
    }

    backtrack(&shapes, &mut grid, &shape_list, 0)
}

fn backtrack(
    shapes: &[Shape],
    grid: &mut Vec<Vec<bool>>,
    to_place: &[usize],
    idx: usize,
) -> bool {
    if idx == to_place.len() {
        return true; // All shapes placed
    }

    let shape_idx = to_place[idx];
    let variations = shapes[shape_idx].rotations_and_flips();

    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else { 0 };

    // Find first empty cell (optimization: place shapes left-to-right, top-to-bottom)
    let mut target_r = None;
    let mut target_c = None;
    'outer: for r in 0..height {
        for c in 0..width {
            if !grid[r][c] {
                target_r = Some(r as i32);
                target_c = Some(c as i32);
                break 'outer;
            }
        }
    }

    let (target_r, target_c) = match (target_r, target_c) {
        (Some(r), Some(c)) => (r, c),
        _ => return true, // Grid is full, all shapes placed
    };

    // Try each variation of the shape
    for var in &variations {
        // Try placing the shape so that one of its cells covers the first empty cell
        for (dr, dc) in &var.cells {
            let place_r = target_r - dr;
            let place_c = target_c - dc;

            if can_place(grid, var, place_r, place_c) {
                place_shape(grid, var, place_r, place_c, true);
                if backtrack(shapes, grid, to_place, idx + 1) {
                    return true;
                }
                place_shape(grid, var, place_r, place_c, false);
            }
        }
    }

    // If no shape can cover the first empty cell, mark it as a "gap" and continue
    // This handles non-exact packing where not all cells need to be filled
    grid[target_r as usize][target_c as usize] = true;
    let result = backtrack(shapes, grid, to_place, idx);
    grid[target_r as usize][target_c as usize] = false;
    result
}

fn can_place(grid: &[Vec<bool>], shape: &Shape, row: i32, col: i32) -> bool {
    let height = grid.len() as i32;
    let width = if height > 0 { grid[0].len() as i32 } else { 0 };

    for (dr, dc) in &shape.cells {
        let r = row + dr;
        let c = col + dc;

        if r < 0 || r >= height || c < 0 || c >= width {
            return false;
        }

        if grid[r as usize][c as usize] {
            return false;
        }
    }

    true
}

fn place_shape(grid: &mut [Vec<bool>], shape: &Shape, row: i32, col: i32, mark: bool) {
    for (dr, dc) in &shape.cells {
        let r = (row + dr) as usize;
        let c = (col + dc) as usize;
        grid[r][c] = mark;
    }
}

fn solve_part1(input: &str) -> usize {
    let (shapes, regions) = parse_input(input);

    let mut count = 0;
    for (i, (w, h, req)) in regions.iter().enumerate() {
        if can_fit_region(&shapes, *w, *h, req) {
            count += 1;
        }
        if (i + 1) % 100 == 0 {
            eprintln!("Processed {}/{} regions, {} successful so far", i + 1, regions.len(), count);
        }
    }
    count
}

fn solve_part2(input: &str) -> i64 {
    let (_shapes, _regions) = parse_input(input);

    // TODO: Implement part 2 once unlocked
    0
}
