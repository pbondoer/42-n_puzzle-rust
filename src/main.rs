extern crate npuzzle_lib;

use std::env;
use std::process;

use npuzzle_lib::*;
use types::Problem;

fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed_puzzle = match args.len() {
        1 => input_parser::parse(None),
        _ => input_parser::parse(Some(&args[1])),
    };
    match parsed_puzzle {
        None => {
            println!("Error while parsing : Exiting");
            process::exit(1);
        }
        Some(puzzle) => {
            let problem = Problem {
                start: puzzle.container,
                end: generator::snail(puzzle.size),
                size: puzzle.size,
            };
            match solver::solve(&problem, heuristics::manhattan) {
                None => println!("No solution found"),
                Some(s) => solver::print_solution(&s),
            }
        }
    }
}
