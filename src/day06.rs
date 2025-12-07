use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/day06.txt").expect("Failed to read input file");

    let grid = parse_grid(&input);
    let problem_ranges = find_problem_ranges(&grid);

    // Part 1: Read numbers horizontally (row by row)
    let total1: u64 = problem_ranges
        .iter()
        .map(|&(start, end)| {
            let (nums, op) = extract_problem_horizontal(&grid, start, end);
            calculate(&nums, op)
        })
        .sum();
    println!("Part 1: {}", total1);

    // Part 2: Read numbers vertically (column by column, right to left)
    let total2: u64 = problem_ranges
        .iter()
        .map(|&(start, end)| {
            let (nums, op) = extract_problem_vertical(&grid, start, end);
            calculate(&nums, op)
        })
        .sum();
    println!("Part 2: {}", total2);
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return vec![];
    }

    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    lines
        .iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.resize(max_len, ' ');
            chars
        })
        .collect()
}

fn find_problem_ranges(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    if grid.is_empty() {
        return vec![];
    }

    let max_len = grid[0].len();

    // Find columns that are entirely spaces (problem separators)
    let mut separator_cols: Vec<usize> = vec![];
    for col in 0..max_len {
        let all_space = grid.iter().all(|row| row[col] == ' ');
        if all_space {
            separator_cols.push(col);
        }
    }

    // Find problem boundaries (contiguous non-separator columns)
    let mut ranges = vec![];
    let mut start_col = None;

    for col in 0..=max_len {
        let is_separator = col == max_len || separator_cols.contains(&col);

        if !is_separator && start_col.is_none() {
            start_col = Some(col);
        } else if is_separator && start_col.is_some() {
            ranges.push((start_col.unwrap(), col));
            start_col = None;
        }
    }

    ranges
}

fn get_operator(grid: &[Vec<char>], start_col: usize, end_col: usize) -> char {
    let operator_row = &grid[grid.len() - 1];
    for col in start_col..end_col {
        let c = operator_row[col];
        if c == '+' || c == '*' {
            return c;
        }
    }
    '+'
}

fn extract_problem_horizontal(grid: &[Vec<char>], start_col: usize, end_col: usize) -> (Vec<u64>, char) {
    let operator = get_operator(grid, start_col, end_col);

    // Extract numbers from rows above the operator row (read horizontally)
    let mut numbers = vec![];
    for row in &grid[..grid.len() - 1] {
        let segment: String = row[start_col..end_col].iter().collect();
        let trimmed = segment.trim();
        if !trimmed.is_empty() {
            if let Ok(num) = trimmed.parse::<u64>() {
                numbers.push(num);
            }
        }
    }

    (numbers, operator)
}

fn extract_problem_vertical(grid: &[Vec<char>], start_col: usize, end_col: usize) -> (Vec<u64>, char) {
    let operator = get_operator(grid, start_col, end_col);

    // Extract numbers by reading columns right-to-left
    // Each column forms a number (top digit = most significant)
    let mut numbers = vec![];

    for col in (start_col..end_col).rev() {
        let mut digits = String::new();
        for row in &grid[..grid.len() - 1] {
            let c = row[col];
            if c.is_ascii_digit() {
                digits.push(c);
            }
        }
        if !digits.is_empty() {
            if let Ok(num) = digits.parse::<u64>() {
                numbers.push(num);
            }
        }
    }

    (numbers, operator)
}

fn calculate(nums: &[u64], op: char) -> u64 {
    match op {
        '+' => nums.iter().sum(),
        '*' => nums.iter().product(),
        _ => 0,
    }
}
