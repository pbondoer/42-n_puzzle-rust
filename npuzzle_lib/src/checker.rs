use types::Atom;
use types::Puzzle;

use util::find_empty_pos;
use util::xy;

fn inversions(puzzle: &Puzzle) -> Atom {
    let mut inversions = 0;
    for i in 0..puzzle.len() - 1 {
        for j in i + 1..puzzle.len() {
            if puzzle[i] == 0 || puzzle[j] == 0 {
                continue;
            }

            if puzzle[i] > puzzle[j] {
                inversions += 1;
            }
        }
    }

    inversions
}

pub fn is_solvable(puzzle: &Puzzle, goal: &Puzzle, size: Atom) -> bool {
    let mut inv = inversions(puzzle);
    let mut g_inv = inversions(goal);

    if size % 2 == 0 {
        inv += xy(find_empty_pos(&puzzle), size).1;
        g_inv += xy(find_empty_pos(&goal), size).1;
    }

    return inv % 2 == g_inv % 2;
}

#[cfg(test)]
mod tests {
    use types::Puzzle;

    #[test]
    fn inversions() {
        let v: Puzzle = vec![4, 3, 2, 1];

        assert_eq!(super::inversions(&v), 6);
    }

    #[test]
    fn inversions_ignore_zero() {
        let v: Puzzle = vec![3, 2, 1, 0];

        assert_eq!(super::inversions(&v), 3);
    }

    #[test]
    fn inversions_none() {
        let v: Puzzle = vec![1, 2, 3, 4];

        assert_eq!(super::inversions(&v), 0);
    }

    #[test]
    fn is_solvable_inverted() {
        let a: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let b: Puzzle = vec![1, 3, 2, 4, 5, 6, 7, 8, 0];

        assert!(!super::is_solvable(&a, &b, 3));
        assert!(!super::is_solvable(&b, &a, 3));
    }

    #[test]
    fn is_solvable_identity() {
        let a: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let b: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];

        assert!(super::is_solvable(&a, &b, 3));
        assert!(super::is_solvable(&b, &a, 3));
    }

    #[test]
    fn is_solvable_4x4() {
        let a: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
        let b: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0, 15];

        assert!(super::is_solvable(&a, &b, 3));
        assert!(super::is_solvable(&b, &a, 3));
    }

    #[test]
    fn is_solvable_4x4_identity() {
        let a: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
        let b: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];

        assert!(super::is_solvable(&a, &b, 3));
        assert!(super::is_solvable(&b, &a, 3));
    }
}
