use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/day03.txt").expect("Failed to read inputs/day03.txt");

    let part1_answer = part1(&input);
    println!("Part 1: {}", part1_answer);

    let part2_answer = part2(&input);
    println!("Part 2: {}", part2_answer);
}

fn max_joltage(bank: &str) -> u32 {
    // Find the maximum two-digit joltage by picking two battery positions
    // The joltage is formed by the digits at those positions in their original order

    let digits: Vec<u32> = bank
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let len = digits.len();
    if len < 2 {
        return 0;
    }

    // Precompute max digit from position i to end (suffix max)
    let mut max_suffix = vec![0u32; len];
    max_suffix[len - 1] = digits[len - 1];
    for i in (0..len - 1).rev() {
        max_suffix[i] = max_suffix[i + 1].max(digits[i]);
    }

    // For each position i as the tens digit, find the max joltage
    let mut max_jolt = 0;
    for i in 0..len - 1 {
        let joltage = 10 * digits[i] + max_suffix[i + 1];
        max_jolt = max_jolt.max(joltage);
    }

    max_jolt
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|bank| max_joltage(bank))
        .sum()
}

fn max_joltage_12(bank: &str) -> u64 {
    // Find the maximum 12-digit joltage by picking exactly 12 battery positions
    // Greedy algorithm: for each position, pick the maximum digit from valid range

    let digits: Vec<u32> = bank
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let n = digits.len();
    if n < 12 {
        return 0;
    }

    let mut result: u64 = 0;
    let mut prev_pos: i32 = -1;

    for i in 0..12 {
        // For digit i, we can pick from positions (prev_pos+1) to (n-12+i)
        // This ensures we have enough positions left for remaining digits
        let start = (prev_pos + 1) as usize;
        let end = n - 12 + i; // inclusive

        // Find the maximum digit in this range (pick leftmost if tied)
        let mut max_val = 0;
        let mut max_pos = start;
        for p in start..=end {
            if digits[p] > max_val {
                max_val = digits[p];
                max_pos = p;
            }
        }

        result = result * 10 + max_val as u64;
        prev_pos = max_pos as i32;
    }

    result
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|bank| max_joltage_12(bank))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_joltage() {
        assert_eq!(max_joltage("987654321111111"), 98);
        assert_eq!(max_joltage("811111111111119"), 89);
        assert_eq!(max_joltage("234234234234278"), 78);
        assert_eq!(max_joltage("818181911112111"), 92);
    }

    #[test]
    fn test_example() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(part1(input), 357);
    }

    #[test]
    fn test_max_joltage_12() {
        assert_eq!(max_joltage_12("987654321111111"), 987654321111);
        assert_eq!(max_joltage_12("811111111111119"), 811111111119);
        assert_eq!(max_joltage_12("234234234234278"), 434234234278);
        assert_eq!(max_joltage_12("818181911112111"), 888911112111);
    }

    #[test]
    fn test_part2_example() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(part2(input), 3121910778619);
    }
}
