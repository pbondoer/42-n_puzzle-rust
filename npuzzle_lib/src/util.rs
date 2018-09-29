use types::Atom;
use types::AtomPair;
use types::Puzzle;

#[inline]
pub fn xy(value: Atom, size: Atom) -> AtomPair {
    (value % size, value / size)
}

#[inline]
pub fn print_puzzle(puzzle: &Puzzle, size: Atom) {
    for i in 0..puzzle.len() {
        print!("{} ", puzzle[i]);

        if i as Atom % size == size - 1 {
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn xy() {
        assert_eq!(super::xy(5, 3), (2, 1));
    }

}
