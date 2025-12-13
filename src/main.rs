mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <day_number|all>", args[0]);
        eprintln!("  day_number: 1-25 to run a specific day");
        eprintln!("  all: run all available puzzles");
        std::process::exit(1);
    }

    let arg = &args[1];

    if arg == "all" {
        run_all();
    } else {
        match arg.parse::<u32>() {
            Ok(day) => run_day(day),
            Err(_) => {
                eprintln!("Invalid argument: {}. Expected a day number (1-25) or 'all'", arg);
                std::process::exit(1);
            }
        }
    }
}

fn run_day(day: u32) {
    println!("=== Day {:02} ===", day);
    match day {
        1 => day01::solve(),
        2 => day02::solve(),
        3 => day03::solve(),
        4 => day04::solve(),
        5 => day05::solve(),
        6 => day06::solve(),
        7 => day07::solve(),
        8 => day08::solve(),
        9 => day09::solve(),
        10 => day10::solve(),
        _ => eprintln!("Day {} not implemented yet", day),
    }
}

fn run_all() {
    println!("Running all implemented puzzles...\n");
    run_day(1);
    println!();
    run_day(2);
    println!();
    run_day(3);
    println!();
    run_day(4);
    println!();
    run_day(5);
    println!();
    run_day(6);
    println!();
    run_day(7);
    println!();
    run_day(8);
    println!();
    run_day(9);
    println!();
    run_day(10);
}
