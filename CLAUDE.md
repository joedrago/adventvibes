# Advent of Code 2025 - Project Memory

## Project Structure
- Rust-based puzzle solver for Advent of Code 2025
- Each day's solution is in its own source file: `src/day01.rs`, `src/day02.rs`, etc.
- Main entry point `src/main.rs` handles CLI arguments to run specific days or all puzzles
- Puzzle inputs stored in `inputs/` directory as text files (e.g., `inputs/day01.txt`)
- Each puzzle has two parts that are solved independently

## Running Puzzles
- Run a specific day: `cargo run -- <day_number>` (e.g., `cargo run -- 1`)
- Run all puzzles: `cargo run -- all`

## Requirements
1. Maintain `JOURNAL.md` with a complete log of all user chat messages
2. Update `CLAUDE.md` with any new requirements or important information
3. Each day's solution prints answers to standard output
4. Solutions should read from input files in `inputs/` directory
5. Git workflow:
   - Always `git push origin` after every commit
   - Commit `CLAUDE.md` and `JOURNAL.md` changes as separate commits
   - Update `JOURNAL.md` before committing when acting on user instructions

## Completed Puzzles
- Day 1 Part 1: Secret Entrance - Count how many times dial lands on 0 (Answer: 1191)
- Day 1 Part 2: Secret Entrance - Count all times dial passes through 0 during rotations (Answer: 6858)
- Day 2 Part 1: Gift Shop - Sum of invalid IDs (repeated digit sequences like 55, 6464) in ranges (Answer: 9188031749)
- Day 2 Part 2: Gift Shop - Sum of invalid IDs (patterns repeated 2+ times like 111, 123123123) (Answer: 11323661261)
- Day 3 Part 1: Lobby - Max joltage from battery banks (pick 2 positions for 2-digit number) (Answer: 17087)
- Day 3 Part 2: Lobby - Max joltage picking 12 positions for 12-digit number (Answer: 169019504359949)
- Day 4 Part 1: Printing Department - Count accessible paper rolls (fewer than 4 adjacent) (Answer: 1480)
- Day 4 Part 2: Printing Department - Iteratively remove accessible rolls until none left (Answer: 8899)
- Day 5 Part 1: Cafeteria - Count available ingredient IDs that fall within fresh ranges (Answer: 707)
- Day 5 Part 2: Cafeteria - Count total unique IDs covered by all fresh ranges (Answer: 361615643045059)
- Day 6 Part 1: Trash Compactor - Parse cephalopod math worksheet horizontally and sum results (Answer: 4309240495780)
- Day 6 Part 2: Trash Compactor - Parse cephalopod math vertically (columns right-to-left) and sum results (Answer: 9170286552289)
- Day 7 Part 1: Laboratories - Count beam splits in tachyon manifold (Answer: 1594)
- Day 7 Part 2: Laboratories - Count total timelines using many-worlds interpretation (Answer: 15650261281478)
- Day 8 Part 1: Playground - Connect 1000 closest junction box pairs, multiply 3 largest circuit sizes (Answer: 50760)
- Day 8 Part 2: Playground - Connect all boxes into one circuit, multiply X coords of last pair (Answer: 3206508875)
- Day 9 Part 1: Movie Theater - Largest rectangle area using two red tiles as corners (Answer: 4760959496)
- Day 9 Part 2: Movie Theater - Largest rectangle with red corners containing only red/green tiles (Answer: 1343576598)
- Day 10 Part 1: Factory - Minimum button presses to match light patterns using GF(2) (Answer: 466)
- Day 10 Part 2: Factory - Minimum button presses to reach joltage targets using ILP (Answer: 17214)
- Day 11 Part 1: Reactor - Count all paths from "you" to "out" through device network (Answer: 613)
- Day 11 Part 2: Reactor - Count paths from "svr" to "out" visiting both "dac" and "fft" (Answer: 372918445876116)
