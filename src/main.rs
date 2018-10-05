extern crate npuzzle_lib;

use std::env;
use std::process;

use npuzzle_lib::*;
use types::ParsedPuzzle;
use types::Problem;
use types::Solver;

/*
 * arguments you can pass:
 * ./rustaquin
 *
 *  --input | -i     >> file, stdin, random
 *  --goal | -o      >> file, stdin, snail, classic
 *  --solver | -s    >> rbfs, astar
 *  --heuristic | -h >> hamming, manhattan, conflicts
 *  --uniform | -u   >> sets h weight to 0
 *  --greedy | -g    >> sets g weight to 0
 *  --adaptive | -a  >> weights change
 *
 *  --               >> stop parsing args
 */
pub struct ParsedArgs {
    pub solver: Solver,
    pub input: String,
    pub goal: String,
    pub iterations: u64,
}

fn parse_args(args: Vec<String>) -> (Problem, ParsedArgs) {
    let mut problem = Problem {
        start: vec![],
        end: vec![],
        size: 0,
        heuristic: heuristics::linear_conflicts,
        g_weight: 1,
        h_weight: 1,
        adaptive: false,
    };

    let mut solver: Solver = solver::astar;

    let mut input: String = "stdin".to_string();
    let mut goal: String = "snail".to_string();
    let mut iterations: u64 = 10000;

    for mut i in 0..args.len() {
        let mut cur = &args[i];

        match &cur as &str {
            "--input" | "-i" => {
                i += 1;
                cur = &args[i];

                input = cur.to_string();
            }
            "--goal" | "-o" => {
                i += 1;
                cur = &args[i];

                goal = cur.to_string();
            }
            "--heuristic" | "-h" => {
                i += 1;
                cur = &args[i];

                match &cur as &str {
                    "hamming" => problem.heuristic = heuristics::hamming,
                    "manhattan" => problem.heuristic = heuristics::manhattan,
                    "conflicts" => problem.heuristic = heuristics::linear_conflicts,
                    _ => {
                        println!("heuristic {} is not valid", cur);
                        process::exit(1);
                    }
                }
            }
            "--solver" | "-s" => {
                i += 1;
                cur = &args[i];

                match &cur as &str {
                    "astar" => solver = solver::astar,
                    "rbfs" => solver = solver::astar, // TODO
                    _ => {
                        println!("solver {} is not valid", cur);
                        process::exit(1);
                    }
                }
            }
            "--adaptive" | "-a" => problem.adaptive = true,
            "--uniform" | "-u" => problem.h_weight = 0,
            "--greedy" | "-g" => problem.g_weight = 0,
            "--iterations" | "-n" => {
                i += 1;
                cur = &args[i];

                match cur.parse::<u64>() {
                    Ok(val) => iterations = val,
                    Err(_) => {
                        println!("iterations {} is not valid", cur);
                        process::exit(1);
                    }
                }
            }
            _ => {}
        }
    }

    (
        problem,
        ParsedArgs {
            solver,
            input,
            goal,
            iterations,
        },
    )
}

fn main() {
    let (mut problem, parsed) = parse_args(env::args().collect());

    // 1. Input
    let opt_input: Option<ParsedPuzzle>;
    let mut random: bool = false;

    match &parsed.input as &str {
        "stdin" => {
            println!("Reading stdin for input state...");
            opt_input = input_parser::parse(None)
        }
        "random" => {
            random = true;
            opt_input = Some(ParsedPuzzle {
                container: vec![],
                size: 3,
            });
        }
        _ => opt_input = input_parser::parse(Some(&parsed.input)),
    }

    let mut input;
    match opt_input {
        Some(e) => input = e,
        None => {
            println!("Error while parsing, exiting");
            process::exit(1);
        }
    }

    // 2. GOAL
    let opt_goal: Option<ParsedPuzzle>;

    match &parsed.goal as &str {
        "stdin" => {
            println!("Reading stdin for goal state...");
            opt_goal = input_parser::parse(None)
        }
        "snail" => {
            opt_goal = Some(ParsedPuzzle {
                container: generator::snail(input.size),
                size: input.size,
            })
        }
        "classic" => {
            opt_goal = Some(ParsedPuzzle {
                container: generator::classic(input.size),
                size: input.size,
            })
        }
        _ => opt_goal = input_parser::parse(Some(&parsed.goal)),
    }

    let goal;
    match opt_goal {
        Some(e) => goal = e,
        None => {
            println!("Error while parsing, exiting");
            process::exit(1);
        }
    }

    // 2.1 Generate random if needed
    if random {
        input = generator::generate_valid_puzzle(&goal, parsed.iterations);
    }

    // 3. Check sizes
    if input.size != goal.size {
        println!(
            "Mismatched puzzle sizes! Expected {}, got {}",
            input.size, goal.size
        );
        process::exit(1);
    }

    // 4. Check solvability
    if !checker::is_solvable(&input.container, &goal.container, input.size) {
        println!("Puzzle not solvable");
        process::exit(1);
    }

    // 5. Solve
    problem.start = input.container;
    problem.end = goal.container;
    problem.size = input.size;

    let solution = (parsed.solver)(&problem);

    if solution.path.len() == 0 {
        println!("Puzzle not solvable");
        process::exit(1);
    }

    solver::print_solution(&solution);
}
