use types::Atom;
use types::Puzzle;

pub fn classic(size: Atom) -> Puzzle {
    let mut solution: Puzzle = (1..size * size + 1).collect();
    let ssize: usize = size as usize * size as usize;

    solution[ssize - 1] = 0;

    solution
}



pub fn snail(size: Atom) -> Puzzle {
    let ssize: usize = size as usize * size as usize;
    let mut solution: Puzzle = vec![0; ssize];

    // right, bottom = 1 ; left, top = -1
    let mut dir: i32 = 1;
    let mut counter = 0;
    let mut x: i32 = -1;
    let mut y: i32 = 0;

    for n in (0..size).rev() {
        for _i in 0..(n + 1) {
            x += dir;
            counter += 1;
            solution[(x + y * size as i32) as usize] = counter;
        }

        for _i in 0..n {
            y += dir;
            counter += 1;
            solution[(x + y * size as i32) as usize] = counter;
        }

        dir = -dir;
    }

    solution[(x + y * size as i32) as usize] = 0;

    solution
}

#[cfg(test)]
mod tests {
    #[test]
    fn classic_3x3() {
        let v = super::classic(3);
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];

        assert_eq!(v, expected);
    }
    #[test]
    fn classic_4x4() {
        let v = super::classic(4);
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];

        assert_eq!(v, expected);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn snail_3x3() {
        let v = super::snail(3);
        let expected = vec![
            1, 2, 3,
            8, 0, 4,
            7, 6, 5,
        ];

        assert_eq!(v, expected);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn snail_4x4() {
        let v = super::snail(4);
        let expected = vec![
            1,   2,  3, 4,
            12, 13, 14, 5,
            11,  0, 15, 6,
            10,  9,  8, 7,
        ];

        assert_eq!(v, expected);
    }
}
