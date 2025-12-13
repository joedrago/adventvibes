use std::collections::HashMap;
use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/day11.txt").expect("Could not read input file");

    let part1 = solve_part1(&input);
    println!("Part 1: {}", part1);

    let part2 = solve_part2(&input);
    println!("Part 2: {}", part2);
}

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() != 2 {
            continue;
        }

        let device = parts[0].to_string();
        let outputs: Vec<String> = parts[1]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        graph.insert(device, outputs);
    }

    graph
}

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    visited: &mut HashMap<String, usize>,
) -> usize {
    // Check if we've already computed this
    if let Some(&count) = visited.get(current) {
        return count;
    }

    // Base case: we've reached the target
    if current == target {
        return 1;
    }

    // If this node has no outputs, there are no paths from here
    let outputs = match graph.get(current) {
        Some(o) => o,
        None => return 0,
    };

    // Count paths through each output
    let mut total_paths = 0;
    for next in outputs {
        total_paths += count_paths(graph, next, target, visited);
    }

    // Memoize the result
    visited.insert(current.to_string(), total_paths);
    total_paths
}

fn solve_part1(input: &str) -> usize {
    let graph = parse_input(input);
    let mut visited = HashMap::new();
    count_paths(&graph, "you", "out", &mut visited)
}

fn count_paths_with_both(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    visited_dac: bool,
    visited_fft: bool,
    memo: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    let key = (current.to_string(), visited_dac, visited_fft);
    if let Some(&count) = memo.get(&key) {
        return count;
    }

    // Update visited flags
    let new_dac = visited_dac || current == "dac";
    let new_fft = visited_fft || current == "fft";

    // Base case: we've reached the target
    if current == target {
        // Only count if we visited both dac and fft
        return if new_dac && new_fft { 1 } else { 0 };
    }

    // If this node has no outputs, there are no paths from here
    let outputs = match graph.get(current) {
        Some(o) => o,
        None => return 0,
    };

    // Count paths through each output
    let mut total_paths = 0;
    for next in outputs {
        total_paths += count_paths_with_both(graph, next, target, new_dac, new_fft, memo);
    }

    // Memoize the result
    memo.insert(key, total_paths);
    total_paths
}

fn solve_part2(input: &str) -> usize {
    let graph = parse_input(input);
    let mut memo = HashMap::new();
    count_paths_with_both(&graph, "svr", "out", false, false, &mut memo)
}
