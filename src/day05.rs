use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/day05.txt").expect("Failed to read input file");

    let (ranges, ids) = parse_input(&input);

    // Part 1: Count fresh ingredients
    let fresh_count = ids.iter().filter(|&&id| is_fresh(id, &ranges)).count();
    println!("Part 1: {}", fresh_count);

    // Part 2: Count total unique fresh IDs across all ranges
    let total_fresh = count_unique_fresh_ids(&ranges);
    println!("Part 2: {}", total_fresh);
}

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let ranges: Vec<(u64, u64)> = parts[0]
        .lines()
        .map(|line| {
            let nums: Vec<u64> = line.split('-').map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();

    let ids: Vec<u64> = parts[1]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    (ranges, ids)
}

fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    ranges.iter().any(|&(start, end)| id >= start && id <= end)
}

fn count_unique_fresh_ids(ranges: &[(u64, u64)]) -> u64 {
    if ranges.is_empty() {
        return 0;
    }

    // Sort ranges by start value
    let mut sorted: Vec<(u64, u64)> = ranges.to_vec();
    sorted.sort_by_key(|r| r.0);

    // Merge overlapping ranges
    let mut merged: Vec<(u64, u64)> = vec![sorted[0]];
    for &(start, end) in &sorted[1..] {
        let last = merged.last_mut().unwrap();
        if start <= last.1 + 1 {
            // Overlapping or adjacent, extend the range
            last.1 = last.1.max(end);
        } else {
            // Non-overlapping, add new range
            merged.push((start, end));
        }
    }

    // Count total IDs in merged ranges
    merged.iter().map(|&(start, end)| end - start + 1).sum()
}
