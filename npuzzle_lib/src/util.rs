use types::Atom;
use types::AtomPair;
use types::Puzzle;

#[inline]
pub fn find_empty_pos(puzzle: &Puzzle) -> Atom {
    for i in 0..puzzle.len() {
        if puzzle[i] == 0 {
            return i as Atom;
        }
    }

    assert!(
        false,
        "find_empty_pos: could not find_empty_pos in {:?}",
        puzzle
    );

    0
}

#[inline]
pub fn xy(value: Atom, size: Atom) -> AtomPair {
    debug_assert!(size > 0, "xy: size cannot be 0");
    debug_assert!(
        value / size < size,
        "xy: y ({}) cannot be >= size ({})",
        value / size,
        size
    );

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
    use types::Atom;

    #[test]
    fn xy() {
        assert_eq!(super::xy(5, 3), (2, 1));
    }

    #[test]
    #[should_panic(expected = "xy: size cannot be 0")]
    fn xy_empty_size() {
        super::xy(2, 0);
    }

    #[test]
    #[should_panic(expected = "xy: y (411) cannot be >= size (3)")]
    fn xy_large_value() {
        super::xy(1234, 3);
    }

    #[test]
    fn find_empty_pos() {
        let v: Vec<Atom> = vec![1, 2, 0, 3];
        assert_eq!(super::find_empty_pos(&v), 2);
    }

    #[test]
    #[should_panic(expected = "find_empty_pos: could not find_empty_pos in [1, 2, 3, 4]")]
    fn find_empty_pos_none() {
        let v: Vec<Atom> = vec![1, 2, 3, 4];
        super::find_empty_pos(&v);
    }
}
