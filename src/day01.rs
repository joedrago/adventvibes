use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/day01.txt").expect("Failed to read inputs/day01.txt");

    let part1_answer = part1(&input);
    println!("Part 1: {}", part1_answer);

    let part2_answer = part2(&input);
    println!("Part 2: {}", part2_answer);
}

fn part1(input: &str) -> u32 {
    let mut position: i32 = 50;
    let mut zero_count: u32 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = &line[0..1];
        let distance: i32 = line[1..].parse().expect("Failed to parse distance");

        match direction {
            "R" => position = (position + distance) % 100,
            "L" => {
                position = position - distance;
                while position < 0 {
                    position += 100;
                }
            }
            _ => panic!("Unknown direction: {}", direction),
        }

        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn part2(input: &str) -> u32 {
    let mut position: i32 = 50;
    let mut zero_count: u32 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = &line[0..1];
        let distance: i32 = line[1..].parse().expect("Failed to parse distance");

        // Count how many times we pass through or land on 0 during this rotation
        // We need to count every "click" that hits 0

        match direction {
            "R" => {
                // Moving right (increasing), wrapping at 100 back to 0
                // Count how many times we cross from 99->0 or pass through 0
                let end_position = (position + distance) % 100;

                // How many full rotations plus any crossing of 0?
                // If we go from position P and travel D steps right:
                // We hit 0 if we pass through it.
                // Number of times we hit 0 = (position + distance) / 100 if position > 0
                // But if position is 0, we don't count the start, only crossings
                // Actually: we count each time we land on 0 during movement
                // From position P going right D steps, we hit 0 at steps: (100-P), (200-P), etc.
                // Number of zeros hit = floor((P + D) / 100) if P > 0
                //                     = floor(D / 100) if P == 0 (since we start there, don't count it)
                // Wait, let me reconsider...

                // Each "click" is one step. We hit 0 whenever our position becomes 0.
                // Going right from P by D steps: positions visited are P+1, P+2, ..., P+D (mod 100)
                // We hit 0 when (P + k) % 100 == 0 for k in 1..=D
                // That means P + k = 100*m for some m >= 1
                // So k = 100*m - P
                // Valid k values: 100-P, 200-P, 300-P, ... as long as k <= D
                // First valid k (if P > 0): 100 - P
                // If P == 0: first valid k is 100

                let first_zero = if position == 0 { 100 } else { 100 - position };
                if distance >= first_zero {
                    // We hit at least one zero
                    // Number of zeros = 1 + (distance - first_zero) / 100
                    zero_count += (1 + (distance - first_zero) / 100) as u32;
                }

                position = end_position;
            }
            "L" => {
                // Moving left (decreasing), wrapping at -1 to 99
                // We hit 0 when position decreases to 0
                // From P going left D steps: positions are P-1, P-2, ..., P-D (mod 100)
                // We hit 0 when (P - k) % 100 == 0 for k in 1..=D
                // If P > 0: first zero at k = P
                // Then every 100 steps after: P, P+100, P+200, ...
                // If P == 0: first zero at k = 100

                let first_zero = if position == 0 { 100 } else { position };
                if distance >= first_zero {
                    zero_count += (1 + (distance - first_zero) / 100) as u32;
                }

                position = ((position - distance) % 100 + 100) % 100;
            }
            _ => panic!("Unknown direction: {}", direction),
        }
    }

    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_example() {
        // From Part 2 example:
        // L68 from 50 -> 82, passes through 0 once
        // L30 from 82 -> 52, no zero
        // R48 from 52 -> 0, lands on zero
        // L5 from 0 -> 95, no zero (starts at 0 but doesn't count start)
        // R60 from 95 -> 55, passes through 0 once
        // L55 from 55 -> 0, lands on zero
        // L1 from 0 -> 99, no zero
        // L99 from 99 -> 0, lands on zero
        // R14 from 0 -> 14, no zero
        // L82 from 14 -> 32, passes through 0 once
        // Total: 6
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";
        assert_eq!(part2(input), 6);
    }

    #[test]
    fn test_r1000_from_50() {
        // R1000 from 50 should hit 0 ten times
        let input = "R1000\n";
        // Starting at 50, going right 1000:
        // First zero at step 50 (position 100 % 100 = 0)
        // Then every 100 steps: 50, 150, 250, 350, 450, 550, 650, 750, 850, 950
        // That's 10 zeros
        assert_eq!(part2(input), 10);
    }
}
