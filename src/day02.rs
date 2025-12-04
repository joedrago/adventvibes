use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/day02.txt").expect("Failed to read inputs/day02.txt");

    let part1_answer = part1(&input);
    println!("Part 1: {}", part1_answer);

    // Part 2 not yet implemented
    // let part2_answer = part2(&input);
    // println!("Part 2: {}", part2_answer);
}

fn is_invalid_id(n: u64) -> bool {
    // An invalid ID is a number where some sequence of digits is repeated twice
    // e.g., 55 = "5" + "5", 6464 = "64" + "64", 123123 = "123" + "123"
    // The number must have an even number of digits and first half == second half

    let s = n.to_string();
    let len = s.len();

    // Must have even number of digits
    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    let first_half = &s[..half];
    let second_half = &s[half..];

    // First half can't start with 0 (no leading zeros in the repeated part)
    // But since n itself doesn't have leading zeros, and first_half is the start of n,
    // first_half won't have leading zeros either

    first_half == second_half
}

fn part1(input: &str) -> u64 {
    let mut sum: u64 = 0;

    // Parse ranges like "11-22,95-115,998-1012" or one per line
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Split by comma for multiple ranges on one line
        for range_str in line.split(',') {
            let range_str = range_str.trim();
            if range_str.is_empty() {
                continue;
            }

            let parts: Vec<&str> = range_str.split('-').collect();
            if parts.len() != 2 {
                continue;
            }

            let start: u64 = parts[0].trim().parse().expect("Failed to parse range start");
            let end: u64 = parts[1].trim().parse().expect("Failed to parse range end");

            for n in start..=end {
                if is_invalid_id(n) {
                    sum += n;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_id() {
        assert!(is_invalid_id(11));
        assert!(is_invalid_id(22));
        assert!(is_invalid_id(55));
        assert!(is_invalid_id(99));
        assert!(is_invalid_id(6464));
        assert!(is_invalid_id(123123));
        assert!(is_invalid_id(1010));

        assert!(!is_invalid_id(12));
        assert!(!is_invalid_id(123));
        assert!(!is_invalid_id(1234));
        assert!(!is_invalid_id(100));
    }

    #[test]
    fn test_example_ranges() {
        // 11-22 contains 11, 22 -> sum = 33
        // 95-115 contains 99 -> sum = 99
        // 998-1012 contains 1010 -> sum = 1010
        // But the example says total is 1227775554 which suggests more ranges

        let input = "11-22";
        // 11, 22 are invalid -> 33
        assert_eq!(part1(input), 11 + 22);
    }
}
