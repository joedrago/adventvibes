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
