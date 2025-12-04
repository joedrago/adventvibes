use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/day02.txt").expect("Failed to read inputs/day02.txt");

    let part1_answer = part1(&input);
    println!("Part 1: {}", part1_answer);

    let part2_answer = part2(&input);
    println!("Part 2: {}", part2_answer);
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

fn is_invalid_id_v2(n: u64) -> bool {
    // Part 2: An invalid ID is a number made of some sequence repeated at least twice
    // e.g., 111 = "1" × 3, 1212121212 = "12" × 5, 123123123 = "123" × 3

    let s = n.to_string();
    let len = s.len();

    // Need at least 2 digits to have a repetition
    if len < 2 {
        return false;
    }

    // Try all possible pattern lengths from 1 to len/2
    // (pattern must repeat at least twice, so max pattern length is len/2)
    for pattern_len in 1..=len / 2 {
        // Length must be divisible by pattern length
        if len % pattern_len != 0 {
            continue;
        }

        let pattern = &s[..pattern_len];

        // Check if the entire string is just this pattern repeated
        let mut is_match = true;
        for i in (pattern_len..len).step_by(pattern_len) {
            if &s[i..i + pattern_len] != pattern {
                is_match = false;
                break;
            }
        }

        if is_match {
            return true;
        }
    }

    false
}

fn part2(input: &str) -> u64 {
    let mut sum: u64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

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
                if is_invalid_id_v2(n) {
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

    #[test]
    fn test_is_invalid_id_v2() {
        // Part 2 examples from puzzle
        assert!(is_invalid_id_v2(11));
        assert!(is_invalid_id_v2(22));
        assert!(is_invalid_id_v2(99));
        assert!(is_invalid_id_v2(111)); // "1" × 3
        assert!(is_invalid_id_v2(999)); // "9" × 3
        assert!(is_invalid_id_v2(1010)); // "10" × 2
        assert!(is_invalid_id_v2(12341234)); // "1234" × 2
        assert!(is_invalid_id_v2(123123123)); // "123" × 3
        assert!(is_invalid_id_v2(1212121212)); // "12" × 5
        assert!(is_invalid_id_v2(1111111)); // "1" × 7
        assert!(is_invalid_id_v2(222222)); // "2" × 6 or "22" × 3 or "222" × 2
        assert!(is_invalid_id_v2(565656)); // "56" × 3
        assert!(is_invalid_id_v2(824824824)); // "824" × 3
        assert!(is_invalid_id_v2(2121212121)); // "21" × 5

        assert!(!is_invalid_id_v2(12));
        assert!(!is_invalid_id_v2(123));
        assert!(!is_invalid_id_v2(1234));
    }

    #[test]
    fn test_part2_example_ranges() {
        // From puzzle example:
        // 95-115 has 99 and 111
        let input = "95-115";
        assert_eq!(part2(input), 99 + 111);

        // 998-1012 has 999 and 1010
        let input2 = "998-1012";
        assert_eq!(part2(input2), 999 + 1010);
    }
}
