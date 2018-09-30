use types::Atom;
use types::AtomPair;
use types::Puzzle;
use types::Result;
use util::xy;

#[inline]
fn dist(a: AtomPair, b: AtomPair) -> Result {
    let x: i32 = a.0 as i32 - b.0 as i32;
    let y: i32 = a.1 as i32 - b.1 as i32;

    (x.abs() + y.abs()) as Result
}

pub fn hamming(a: &Puzzle, b: &Puzzle, _size: Atom) -> Result {
    let mut total: Result = 0;

    if a == b {
        return 0;
    }

    for i in 0..a.len() {
        let a_val = a[i];
        let b_val = b[i];

        if a_val != b_val {
            total += 1;
        }
    }

    total
}

pub fn manhattan(a: &Puzzle, b: &Puzzle, size: Atom) -> Result {
    let mut total: Result = 0;

    if a == b {
        return 0;
    }

    for i in 0..a.len() {
        let a_val = a[i];
        let a_pos = xy(i as Atom, size);

        for j in 0..b.len() {
            let b_val = b[j];

            if a_val == b_val {
                let b_pos = xy(j as Atom, size);
                total += dist(a_pos, b_pos);
            }
        }
    }

    total
}

// I know it's horrible, but I can't think of a better way right now
// Also, it works so that's good enough(tm)

pub fn linear_conflicts(a: &Puzzle, b: &Puzzle, size: Atom) -> Result {
    let mut penalty = 0;

    for row in 0..size {
        for x in 0..size - 1 {
            for x_2 in (x + 1)..size {
                let pos = x + row * size;
                let pos_2 = x_2 + row * size;

                let val = a[pos as usize];
                let val_2 = a[pos_2 as usize];

                let mut goal = (0, 0);
                let mut goal_2 = (0, 0);
                let mut changed = 0;

                for i in 0..b.len() {
                    if b[i] == val {
                        goal = xy(i as Atom, size);
                        changed += 1;
                    }

                    if b[i] == val_2 {
                        goal_2 = xy(i as Atom, size);
                        changed += 1;
                    }
                }

                debug_assert!(changed == 2);

                if goal.1 == row && goal_2.1 == row {
                    if goal.0 > goal_2.0 {
                        penalty += 1;
                    }
                }
            }
        }
    }

    for col in 0..size {
        for y in 0..size - 1 {
            for y_2 in (y + 1)..size {
                let pos = col + y * size;
                let pos_2 = col + y_2 * size;

                let val = a[pos as usize];
                let val_2 = a[pos_2 as usize];

                let mut goal = (0, 0);
                let mut goal_2 = (0, 0);

                let mut changed = 0;

                for i in 0..b.len() {
                    if b[i] == val {
                        goal = xy(i as Atom, size);
                        changed += 1;
                    }

                    if b[i] == val_2 {
                        goal_2 = xy(i as Atom, size);
                        changed += 1;
                    }
                }

                debug_assert!(changed == 2);

                if goal.0 == col && goal_2.0 == col {
                    if goal.1 > goal_2.1 {
                        penalty += 1;
                    }
                }
            }
        }
    }

    manhattan(a, b, size) + penalty
}

#[cfg(test)]
mod tests {
    use types::Atom;

    #[test]
    fn dist() {
        assert_eq!(super::dist((3, 3), (5, 5)), 4);
        assert_eq!(super::dist((5, 5), (3, 3)), 4);
        assert_eq!(super::dist((1, 1), (1, 1)), 0);
    }

    #[test]
    fn hamming() {
        let a: Vec<Atom> = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let b: Vec<Atom> = vec![8, 3, 1, 4, 7, 2, 5, 0, 6];

        let expected = 8;

        assert_eq!(super::hamming(&a, &b, 3), expected);
        assert_eq!(super::hamming(&b, &a, 3), expected);
    }

    #[test]
    fn hamming_identity() {
        let a: Vec<Atom> = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let b: Vec<Atom> = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];

        let expected = 0;

        assert_eq!(super::hamming(&a, &b, 3), expected);
        assert_eq!(super::hamming(&b, &a, 3), expected);
    }

    #[test]
    fn manhattan() {
        let a: Vec<Atom> = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let b: Vec<Atom> = vec![8, 3, 1, 4, 7, 2, 5, 0, 6];

        let expected = 14;

        assert_eq!(super::manhattan(&a, &b, 3), expected);
        assert_eq!(super::manhattan(&b, &a, 3), expected);
    }

    #[test]
    fn manhattan_identity() {
        let a: Vec<Atom> = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let b: Vec<Atom> = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];

        let expected = 0;

        assert_eq!(super::manhattan(&a, &b, 3), expected);
        assert_eq!(super::manhattan(&b, &a, 3), expected);
    }

    #[test]
    fn linear_conflicts() {
        let a: Vec<Atom> = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let b: Vec<Atom> = vec![8, 3, 1, 4, 7, 2, 5, 0, 6];

        let expected = 15;

        assert_eq!(super::linear_conflicts(&a, &b, 3), expected);
        assert_eq!(super::linear_conflicts(&b, &a, 3), expected);
    }

    #[test]
    fn linear_conflicts_x() {
        let a: Vec<Atom> = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let b: Vec<Atom> = vec![3, 2, 1, 4, 5, 6, 7, 8, 0];

        let expected = 7;

        assert_eq!(super::linear_conflicts(&a, &b, 3), expected);
        assert_eq!(super::linear_conflicts(&b, &a, 3), expected);
    }

    #[test]
    fn linear_conflicts_y() {
        let a: Vec<Atom> = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let b: Vec<Atom> = vec![7, 2, 3, 4, 5, 6, 1, 8, 0];

        let expected = 7;

        assert_eq!(super::linear_conflicts(&a, &b, 3), expected);
        assert_eq!(super::linear_conflicts(&b, &a, 3), expected);
    }

    #[test]
    fn linear_conflicts_identity() {
        let a: Vec<Atom> = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let b: Vec<Atom> = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];

        let expected = 0;

        assert_eq!(super::linear_conflicts(&a, &b, 3), expected);
        assert_eq!(super::linear_conflicts(&b, &a, 3), expected);
    }
}
