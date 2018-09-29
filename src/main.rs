extern crate npuzzle_lib;

use npuzzle_lib::*;

fn main() {
    solver::solve(&types::Problem {
        start: vec![1, 2, 3, 4, 5, 6, 7, 8, 0],
        end: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
        size: 3
    });
}
