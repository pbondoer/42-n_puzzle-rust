// Types
pub type Atom = u16;
pub type Result = u64;

pub type Puzzle = Vec<Atom>;
pub type AtomPair = (Atom, Atom);

// Constants
pub static MAX_PUZZLE_SIZE: Atom = 100;
pub static MAX_ARRAY_SIZE: Atom = MAX_PUZZLE_SIZE * MAX_PUZZLE_SIZE;

pub struct Node<'a> {
    pub array: Puzzle,
    pub h_result: Result,
    pub g_result: Result,
    pub links: Vec<&'a Node<'a>>,
}

pub struct Problem {
    pub start_state: Puzzle,
    pub end_state: Puzzle,
    pub size: Atom,
}
