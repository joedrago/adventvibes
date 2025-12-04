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
