mod day01;
mod day02;

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
        _ => eprintln!("Day {} not implemented yet", day),
    }
}

fn run_all() {
    println!("Running all implemented puzzles...\n");
    run_day(1);
    println!();
    run_day(2);
}
