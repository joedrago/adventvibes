use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/day07.txt").expect("Failed to read input file");

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // Part 1: Count how many times the beam is split
    let splits = count_splits(&grid);
    println!("Part 1: {}", splits);

    // Part 2: Count total timelines (each split doubles timelines on that path)
    let timelines = count_timelines(&grid);
    println!("Part 2: {}", timelines);
}

fn count_splits(grid: &[Vec<char>]) -> usize {
    if grid.is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();

    // Find starting position 'S'
    let mut start_row = 0;
    let mut start_col = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start_row = r;
                start_col = c;
            }
        }
    }

    // BFS to simulate beams - each beam has (row, col) and moves downward
    // When splitting, it creates beams at col-1 and col+1
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    // Beam starts at S and moves downward
    queue.push_back((start_row, start_col));

    let mut split_count = 0;

    while let Some((row, col)) = queue.pop_front() {
        // Check if this position was already visited
        if visited.contains(&(row, col)) {
            continue;
        }
        visited.insert((row, col));

        // Move downward
        if row + 1 >= rows {
            continue;
        }
        let next_row = row + 1;
        let cell = grid[next_row][col];

        match cell {
            '.' | 'S' => {
                // Continue downward
                queue.push_back((next_row, col));
            }
            '^' => {
                // Splitter: count split, create left and right beams
                split_count += 1;
                if col > 0 {
                    queue.push_back((next_row, col - 1));
                }
                if col + 1 < cols {
                    queue.push_back((next_row, col + 1));
                }
            }
            _ => {
                // Unknown cell, continue downward
                queue.push_back((next_row, col));
            }
        }
    }

    split_count
}

fn count_timelines(grid: &[Vec<char>]) -> u64 {
    if grid.is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();

    // Find starting position 'S'
    let mut start_col = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start_col = c;
                break;
            }
        }
    }

    // Track number of timelines at each column position
    // Process row by row
    let mut timelines: HashMap<usize, u64> = HashMap::new();
    timelines.insert(start_col, 1);

    for row in 0..rows - 1 {
        let mut next_timelines: HashMap<usize, u64> = HashMap::new();

        for (&col, &count) in &timelines {
            let cell = grid[row + 1][col];

            match cell {
                '.' | 'S' => {
                    // Continue downward, timelines stay the same
                    *next_timelines.entry(col).or_insert(0) += count;
                }
                '^' => {
                    // Split: each timeline becomes 2 (left and right)
                    if col > 0 {
                        *next_timelines.entry(col - 1).or_insert(0) += count;
                    }
                    if col + 1 < cols {
                        *next_timelines.entry(col + 1).or_insert(0) += count;
                    }
                }
                _ => {
                    // Unknown cell, continue downward
                    *next_timelines.entry(col).or_insert(0) += count;
                }
            }
        }

        timelines = next_timelines;
    }

    // Sum all timelines that reached the bottom
    timelines.values().sum()
}
