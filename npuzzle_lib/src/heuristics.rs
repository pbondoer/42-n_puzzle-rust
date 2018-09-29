use types::Atom;
use types::AtomPair;
use types::Puzzle;
use util::xy;

#[inline]
fn dist(a: AtomPair, b: AtomPair) -> u64 {
    let x: i32 = a.0 as i32 - b.0 as i32;
    let y: i32 = a.1 as i32 - b.1 as i32;

    (x.abs() + y.abs()) as u64
}

pub fn manhattan(node_a: &Puzzle, node_b: &Puzzle, size: Atom) -> u64 {
    let mut total: u64 = 0;

    assert_eq!(
        node_a.len(),
        node_b.len(),
        "manhattan: puzzles not of same size"
    );
    assert_eq!(
        node_a.len() % size as usize,
        0,
        "manhattan {} % {} != 0",
        node_a.len(),
        size
    );

    for i in 0..node_a.len() {
        let a = node_a[i];
        let a_pos = xy(i as Atom, size);

        for j in 0..node_b.len() {
            let b = node_b[j];

            if a == b {
                let b_pos = xy(j as Atom, size);
                total += dist(a_pos, b_pos);
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use types::Node;

    #[test]
    fn dist() {
        assert_eq!(super::dist((3, 3), (5, 5)), 4);
        assert_eq!(super::dist((5, 5), (3, 3)), 4);
    }

    #[test]
    fn manhattan() {
        let node_a = Node {
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 0],
            h_result: 0,
            g_result: 0,
        };

        let node_b = Node {
            array: vec![8, 3, 1, 4, 7, 2, 5, 0, 6],
            h_result: 0,
            g_result: 0,
        };

        assert_eq!(super::manhattan(&node_a.array, &node_b.array, 3), 14);
    }

    #[test]
    fn manhattan_same() {
        let node_a = Node {
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 0],
            h_result: 0,
            g_result: 0,
            links: Vec::new(),
        };

        let node_b = Node {
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 0],
            h_result: 0,
            g_result: 0,
            links: Vec::new(),
        };

        assert_eq!(super::manhattan(&node_a.array, &node_b.array, 3), 0);
    }
}
