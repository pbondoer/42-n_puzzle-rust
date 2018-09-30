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
                print!("[{}, {}] ", puzzle[i], puzzle[j]);
                inversions += 1;
            }
        }
        println!("");
    }

    println!("");

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
