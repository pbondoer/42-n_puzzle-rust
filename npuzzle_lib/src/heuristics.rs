use node::Node;
use node::Atom;
use node::AtomPair;
use util::xy;

#[inline]
fn dist(a: AtomPair, b: AtomPair) -> u64 {
    let x: i32 = a.0 as i32 - b.0 as i32;
    let y: i32 = a.1 as i32 - b.1 as i32;

    (x.abs() + y.abs()) as u64
}

pub fn manhattan(node_a : &Node, node_b : &Node, size: Atom) -> u64 {
    let mut total: u64 = 0;

    for i in 0..node_a.array.len() {
        let a = node_a.array[i];
        let a_pos = xy(i as Atom, size);

        for j in 0..node_b.array.len() {
            let b = node_b.array[j];

            if a == b {
                let b_pos = xy(j as Atom, size);
                total += dist(a_pos, b_pos);
            }
        }
    }

    total
}