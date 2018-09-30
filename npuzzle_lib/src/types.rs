use std::cmp::Ordering;

// Types
pub type Atom = u16;
pub type Result = u64;

pub type Puzzle = Vec<Atom>;
pub type AtomPair = (Atom, Atom);

// Constants
pub static MAX_PUZZLE_SIZE: Atom = 100;
pub static MAX_ARRAY_SIZE: Atom = MAX_PUZZLE_SIZE * MAX_PUZZLE_SIZE;

#[derive(Eq, PartialEq, Hash)]
pub struct Node {
    pub array: Puzzle,
    pub pos: Atom,
    pub h_result: Result,
    pub g_result: Result,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        let total = self.h_result + self.g_result;
        let total_other = other.h_result + other.g_result;

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
}

pub struct ParsedPuzzle {
    pub container: Puzzle,
    pub size: Atom,
}
