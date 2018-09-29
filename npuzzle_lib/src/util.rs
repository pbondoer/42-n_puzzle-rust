use node::Atom;
use node::AtomPair;

#[inline]
pub fn xy(value: Atom, size: Atom) -> AtomPair {
    (value % size, value / size)
}

#[test]
fn test_xy() {
    assert_eq!(xy(5, 3), (2, 1));
}