use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/day10.txt").expect("Could not read input file");

    let part1 = solve_part1(&input);
    println!("Part 1: {}", part1);

    let part2 = solve_part2(&input);
    println!("Part 2: {}", part2);
}

#[derive(Debug)]
struct Machine {
    target: Vec<bool>, // true = on (#), false = off (.)
    buttons: Vec<Vec<usize>>, // each button toggles these positions
    joltages: Vec<i64>, // joltage requirements for part 2
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            // Parse pattern in brackets [.##.]
            let pattern_start = line.find('[').unwrap();
            let pattern_end = line.find(']').unwrap();
            let pattern = &line[pattern_start + 1..pattern_end];
            let target: Vec<bool> = pattern.chars().map(|c| c == '#').collect();

            // Parse buttons (0,3,4) (1,3) etc. and joltages {3,5,4,7}
            let rest = &line[pattern_end + 1..];
            let mut buttons = Vec::new();
            let mut joltages = Vec::new();

            let mut i = 0;
            while i < rest.len() {
                if rest.chars().nth(i) == Some('(') {
                    let end = rest[i..].find(')').unwrap() + i;
                    let button_str = &rest[i + 1..end];
                    if !button_str.is_empty() {
                        let positions: Vec<usize> = button_str
                            .split(',')
                            .map(|s| s.trim().parse().unwrap())
                            .collect();
                        buttons.push(positions);
                    }
                    i = end + 1;
                } else if rest.chars().nth(i) == Some('{') {
                    let end = rest[i..].find('}').unwrap() + i;
                    let joltage_str = &rest[i + 1..end];
                    joltages = joltage_str
                        .split(',')
                        .map(|s| s.trim().parse().unwrap())
                        .collect();
                    break;
                } else {
                    i += 1;
                }
            }

            Machine { target, buttons, joltages }
        })
        .collect()
}

// Solve using Gaussian elimination over GF(2) with optimization for minimum presses
fn solve_machine(machine: &Machine) -> Option<usize> {
    let n_lights = machine.target.len();
    let n_buttons = machine.buttons.len();

    // Build augmented matrix for system of linear equations over GF(2)
    let mut matrix = vec![vec![false; n_buttons + 1]; n_lights];

    // Fill matrix: matrix[light][button] = true if button toggles that light
    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for &light_pos in button {
            if light_pos < n_lights {
                matrix[light_pos][button_idx] = true;
            }
        }
    }

    // Set target column
    for (i, &target_val) in machine.target.iter().enumerate() {
        matrix[i][n_buttons] = target_val;
    }

    // Gaussian elimination over GF(2) - reduced row echelon form
    let mut pivot_cols = Vec::new();
    let mut current_row = 0;

    for col in 0..n_buttons {
        // Find pivot in this column
        let mut pivot_row = None;
        for row in current_row..n_lights {
            if matrix[row][col] {
                pivot_row = Some(row);
                break;
            }
        }

        if let Some(pr) = pivot_row {
            // Swap rows if needed
            if pr != current_row {
                matrix.swap(current_row, pr);
            }

            pivot_cols.push(col);

            // Eliminate all other rows
            for row in 0..n_lights {
                if row != current_row && matrix[row][col] {
                    for c in 0..=n_buttons {
                        matrix[row][c] ^= matrix[current_row][c];
                    }
                }
            }
            current_row += 1;
        }
    }

    // Check for inconsistency
    for row in &matrix {
        let all_zeros = row[..n_buttons].iter().all(|&x| !x);
        if all_zeros && row[n_buttons] {
            return None; // No solution
        }
    }

    // Find free variables (columns without pivots)
    let mut is_pivot = vec![false; n_buttons];
    for &col in &pivot_cols {
        is_pivot[col] = true;
    }
    let free_vars: Vec<usize> = (0..n_buttons).filter(|&i| !is_pivot[i]).collect();

    // Try all combinations of free variables to find minimum presses
    let num_free = free_vars.len();
    let mut min_presses = usize::MAX;

    for mask in 0..(1 << num_free) {
        let mut solution = vec![false; n_buttons];

        // Set free variables according to mask
        for (i, &var) in free_vars.iter().enumerate() {
            solution[var] = (mask >> i) & 1 == 1;
        }

        // Solve for pivot variables using back substitution
        for i in (0..current_row).rev() {
            let pivot_col = pivot_cols[i];
            let mut val = matrix[i][n_buttons];
            for c in (pivot_col + 1)..n_buttons {
                if matrix[i][c] {
                    val ^= solution[c];
                }
            }
            solution[pivot_col] = val;
        }

        // Count presses for this solution
        let presses = solution.iter().filter(|&&x| x).count();
        min_presses = min_presses.min(presses);
    }

    Some(min_presses)
}

fn solve_part1(input: &str) -> usize {
    let machines = parse_input(input);

    machines
        .iter()
        .filter_map(|machine| solve_machine(machine))
        .sum()
}

fn solve_part2(input: &str) -> i64 {
    let machines = parse_input(input);

    machines
        .iter()
        .filter_map(|machine| solve_joltage(machine))
        .sum()
}

// Solve joltage configuration using integer linear programming
fn solve_joltage(machine: &Machine) -> Option<i64> {
    let n_counters = machine.joltages.len();
    let n_buttons = machine.buttons.len();

    if n_counters == 0 {
        return Some(0);
    }

    // Build matrix: matrix[counter][button] = 1 if button affects counter
    let mut matrix = vec![vec![0i64; n_buttons + 1]; n_counters];

    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for &counter_pos in button {
            if counter_pos < n_counters {
                matrix[counter_pos][button_idx] = 1;
            }
        }
    }

    // Set target column
    for (i, &joltage) in machine.joltages.iter().enumerate() {
        matrix[i][n_buttons] = joltage;
    }

    // Gaussian elimination (keep as integers, don't normalize)
    let mut pivot_cols = Vec::new();
    let mut pivot_vals = Vec::new();
    let mut current_row = 0;

    for col in 0..n_buttons {
        // Find pivot (non-zero element)
        let mut pivot_row = None;
        for row in current_row..n_counters {
            if matrix[row][col] != 0 {
                pivot_row = Some(row);
                break;
            }
        }

        if let Some(pr) = pivot_row {
            if pr != current_row {
                matrix.swap(current_row, pr);
            }

            pivot_cols.push(col);
            pivot_vals.push(matrix[current_row][col]);

            // Eliminate rows below (but not above for now)
            for row in (current_row + 1)..n_counters {
                if matrix[row][col] != 0 {
                    let pivot_val = matrix[current_row][col];
                    let factor = matrix[row][col];
                    for c in 0..=n_buttons {
                        matrix[row][c] = matrix[row][c] * pivot_val - matrix[current_row][c] * factor;
                    }
                }
            }
            current_row += 1;
        }
    }

    // Check for inconsistency
    for row in &matrix {
        let all_zeros = row[..n_buttons].iter().all(|&x| x == 0);
        if all_zeros && row[n_buttons] != 0 {
            return None; // No solution
        }
    }

    // Find free variables
    let mut is_pivot = vec![false; n_buttons];
    for &col in &pivot_cols {
        is_pivot[col] = true;
    }
    let free_vars: Vec<usize> = (0..n_buttons).filter(|&i| !is_pivot[i]).collect();

    // Compute bounds for free variables based on constraints
    let max_joltage = machine.joltages.iter().max().unwrap_or(&0);
    let max_free = if free_vars.is_empty() {
        0
    } else {
        // Be more generous with the bound
        (*max_joltage * n_buttons as i64) as usize
    };

    let mut min_presses = i64::MAX;

    // Try combinations of free variables
    search_free_vars_v2(&matrix, &pivot_cols, &pivot_vals, &free_vars, current_row, n_buttons,
                        0, &mut vec![0; n_buttons], &mut min_presses, max_free);

    if min_presses == i64::MAX {
        None
    } else {
        Some(min_presses)
    }
}

fn search_free_vars_v2(
    matrix: &[Vec<i64>],
    pivot_cols: &[usize],
    pivot_vals: &[i64],
    free_vars: &[usize],
    num_pivots: usize,
    n_buttons: usize,
    var_idx: usize,
    current: &mut Vec<i64>,
    min_presses: &mut i64,
    max_val: usize,
) {
    if var_idx == free_vars.len() {
        // All free variables set, solve for pivot variables via back substitution
        let mut solution = current.clone();

        // Back substitution
        for i in (0..num_pivots).rev() {
            let pivot_col = pivot_cols[i];
            let pivot_val = pivot_vals[i];

            let mut rhs = matrix[i][n_buttons];
            for c in 0..n_buttons {
                if c != pivot_col {
                    rhs -= matrix[i][c] * solution[c];
                }
            }

            // Check if divisible
            if rhs % pivot_val != 0 {
                return; // Not a valid integer solution
            }

            solution[pivot_col] = rhs / pivot_val;
        }

        // Check if all values are non-negative
        if solution.iter().all(|&x| x >= 0) {
            let sum: i64 = solution.iter().sum();
            *min_presses = (*min_presses).min(sum);
        }
        return;
    }

    // Try different values for this free variable
    let var = free_vars[var_idx];
    for val in 0..=max_val {
        current[var] = val as i64;

        // Prune: if current sum already exceeds minimum, stop
        let current_sum: i64 = current.iter().sum();
        if current_sum >= *min_presses {
            break;
        }

        search_free_vars_v2(matrix, pivot_cols, pivot_vals, free_vars, num_pivots, n_buttons,
                           var_idx + 1, current, min_presses, max_val);
    }
    current[var] = 0;
}
