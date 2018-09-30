extern crate npuzzle_lib;

use npuzzle_lib::*;

#[cfg_attr(rustfmt, rustfmt_skip)]
fn main() {
    let problem = types::Problem {
        start: vec![
            1, 2, 3,
            4, 0, 6,
            7, 8, 5,
        ],
        end: vec![
            1, 2, 3,
            8, 0, 4,
            7, 6, 5,
        ],
        size: 3,
    };
    //let solution = solver::solve(&problem, heuristics::linear_conflicts);
    let solution = None;

    match solution {
        None => println!("No solution found"),
        Some(s) => solver::print_solution(&s),
    }
}
