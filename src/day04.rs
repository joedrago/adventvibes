use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/day04.txt").expect("Failed to read inputs/day04.txt");

    let part1_answer = part1(&input);
    println!("Part 1: {}", part1_answer);

    let part2_answer = part2(&input);
    println!("Part 2: {}", part2_answer);
}

fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();

    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' {
                // Count adjacent @ symbols in all 8 directions
                let adjacent = count_adjacent(&grid, r, c, rows, cols);
                if adjacent < 4 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_adjacent(grid: &[Vec<char>], r: usize, c: usize, rows: usize, cols: usize) -> usize {
    let directions: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let mut count = 0;
    for (dr, dc) in directions {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;

        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
            if grid[nr as usize][nc as usize] == '@' {
                count += 1;
            }
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();

    let mut total_removed = 0;

    loop {
        // Find all accessible rolls (fewer than 4 adjacent)
        let mut to_remove = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '@' {
                    let adjacent = count_adjacent(&grid, r, c, rows, cols);
                    if adjacent < 4 {
                        to_remove.push((r, c));
                    }
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        // Remove all accessible rolls
        for (r, c) in &to_remove {
            grid[*r][*c] = '.';
        }

        total_removed += to_remove.len();
    }

    total_removed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_example() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        assert_eq!(part2(input), 43);
    }
}
