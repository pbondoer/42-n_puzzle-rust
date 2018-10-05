use std::cmp::Ordering;
use std::time::SystemTime;

// Types
pub type Atom = u16;
pub type Statistic = usize;
pub type Result = u64;

pub type Puzzle = Vec<Atom>;
pub type AtomPair = (Atom, Atom);

pub type Heuristic = fn(a: &Puzzle, b: &Puzzle, size: Atom) -> Result;
pub type Solver = fn(p: &Problem) -> Solution;

// Constants
pub static MAX_PUZZLE_SIZE: Atom = 100;
pub static MAX_ARRAY_SIZE: Atom = MAX_PUZZLE_SIZE * MAX_PUZZLE_SIZE;

#[derive(Eq, PartialEq, Hash)]
pub struct Node {
    pub array: Puzzle,
    pub pos: Atom,
    pub h_result: Result,
    pub g_result: Result,
    pub f_result: Result,
}

pub struct Solution<'a> {
    pub problem: &'a Problem,
    pub path: Vec<Puzzle>,
    pub max_states: Statistic,
    pub opened_states: Statistic,
    pub current_open_states: Statistic,
    pub closed_states: Statistic,
    pub time: SystemTime,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        let total = self.f_result;
        let total_other = other.f_result;

        total_other.cmp(&total)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Problem {
    pub start: Puzzle,
    pub end: Puzzle,
    pub size: Atom,
    pub heuristic: Heuristic,
    pub g_weight: Result,
    pub h_weight: Result,
}

pub struct ParsedPuzzle {
    pub container: Puzzle,
    pub size: Atom,
}
