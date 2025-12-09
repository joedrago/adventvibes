use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/day08.txt").expect("Could not read input file");

    let part1 = solve_part1(&input);
    println!("Part 1: {}", part1);

    let part2 = solve_part2(&input);
    println!("Part 2: {}", part2);
}

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn distance_squared(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            // Union by rank
            if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            } else if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            } else {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
                self.rank[root_x] += 1;
            }
        }
    }

    fn get_component_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut sizes = Vec::new();
        for i in 0..n {
            if self.find(i) == i {
                sizes.push(self.size[i]);
            }
        }
        sizes
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<f64> = line
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            Point {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect()
}

fn solve_part1(input: &str) -> u64 {
    let points = parse_input(input);
    let n = points.len();

    // Calculate all pairwise distances
    let mut distances: Vec<(f64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist_sq = points[i].distance_squared(&points[j]);
            distances.push((dist_sq, i, j));
        }
    }

    // Sort by distance (squared is fine for comparison)
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Connect the 1000 closest pairs
    let mut uf = UnionFind::new(n);
    for (_, i, j) in distances.iter().take(1000) {
        uf.union(*i, *j);
    }

    // Get component sizes and find the three largest
    let mut sizes = uf.get_component_sizes();
    sizes.sort_by(|a, b| b.cmp(a)); // Sort descending

    // Multiply the three largest
    let result: u64 = sizes.iter().take(3).map(|&s| s as u64).product();
    result
}

fn solve_part2(input: &str) -> u64 {
    let points = parse_input(input);
    let n = points.len();

    // Calculate all pairwise distances
    let mut distances: Vec<(f64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist_sq = points[i].distance_squared(&points[j]);
            distances.push((dist_sq, i, j));
        }
    }

    // Sort by distance (squared is fine for comparison)
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Connect pairs until all are in one circuit
    let mut uf = UnionFind::new(n);
    let mut num_components = n;

    for (_, i, j) in distances.iter() {
        let root_i = uf.find(*i);
        let root_j = uf.find(*j);

        if root_i != root_j {
            uf.union(*i, *j);
            num_components -= 1;

            // Check if we just unified everything
            if num_components == 1 {
                // This is the last connection - multiply X coordinates
                let x1 = points[*i].x as u64;
                let x2 = points[*j].x as u64;
                return x1 * x2;
            }
        }
    }

    0 // Should never reach here if input is valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Example from puzzle: 10 connections result in circuits of size 5, 4, 2
        // Product = 40
        let input = "0,0,0
1,0,0
2,0,0
3,0,0
4,0,0
10,0,0
11,0,0
12,0,0
13,0,0
20,0,0
21,0,0
";
        // With 10 connections on 11 points along a line, we'd connect
        // the closest pairs creating specific circuits
        // This is a simplified test case
    }
}
