use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/day03.txt").expect("Failed to read inputs/day03.txt");

    let part1_answer = part1(&input);
    println!("Part 1: {}", part1_answer);

    // Part 2 not yet implemented
    // let part2_answer = part2(&input);
    // println!("Part 2: {}", part2_answer);
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
}
