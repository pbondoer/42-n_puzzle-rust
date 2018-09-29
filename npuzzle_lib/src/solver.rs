use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use types::Atom;
use types::Node;
use types::Problem;
use types::Puzzle;

use util::print_puzzle;
use util::xy;

use heuristics::manhattan;

const NEIGHBOR_DELTAS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn neighbors(puzzle: &Puzzle, pos: Atom, size: Atom) -> HashSet<(Puzzle, Atom)> {
    let mut set = HashSet::new();

    for p in NEIGHBOR_DELTAS.iter() {
        let tuple = xy(pos, size);

        let mut x: i32 = tuple.0 as i32;
        let mut y: i32 = tuple.1 as i32;

        // check out of bounds
        x += p.0 as i32;
        if x < 0 || x >= size as i32 {
            continue;
        }
        y += p.1 as i32;
        if y < 0 || y >= size as i32 {
            continue;
        }

        // clone and swap
        let mut cur = puzzle.clone();
        let cur_pos = y as i32 * size as i32 + x as i32;
        cur.swap(pos as usize, cur_pos as usize);

        set.insert((cur, cur_pos as u16));
    }

    assert!(!set.is_empty(), "set should not be empty");

    set
}

fn find_empty_pos(puzzle: &Puzzle) -> Atom {
    for i in 0..puzzle.len() {
        if puzzle[i] == 0 {
            return i as Atom;
        }
    }

    assert!(false "could not find empty pos");

    0
}

pub fn solve(problem: &Problem) {
    println!("-----------------------------");
    println!("start state:");
    print_puzzle(&problem.start, problem.size);
    println!("");
    println!("end state:");
    print_puzzle(&problem.end, problem.size);
    println!("");

    println!("size: {}", problem.size);
    println!("-----------------------------");

    let mut open = BinaryHeap::new();
    let mut closed = HashSet::new();
    let mut from = HashMap::new();

    open.push(Node {
        array: problem.start.clone(),
        h_result: manhattan(&problem.start, &problem.end, problem.size),
        g_result: 0,
        pos: find_empty_pos(&problem.start),
    });

    let mut node_wrapped = open.pop();
    let mut node: Node;

    while node_wrapped != None {
        node = node_wrapped.unwrap();

        println!("open size: {}", open.len());
        println!("-----------------------------");
        print_puzzle(&node.array, problem.size);
        println!("-----------------------------");

        if node.array == problem.end {
            println!("Found solution, breaking the loop");
            break;
        }

        closed.insert(node.array.clone());

        for raw_neighbor in neighbors(&node.array, node.pos, problem.size) {
            let (neighbor, neighbor_pos) = raw_neighbor;

            //println!("    raw neighbor: ");
            //print_puzzle(&neighbor, problem.size);

            if closed.contains(&neighbor) {
                continue;
            }

            open.push(Node {
                array: neighbor.clone(),
                h_result: manhattan(&neighbor, &problem.end, problem.size),
                g_result: node.g_result + 1,
                pos: neighbor_pos,
            });

            //println!("    h_result: {}", manhattan(&neighbor, &problem.end, problem.size));

            //println!("----------------");
            from.insert(neighbor, node.array.clone());
        }

        node_wrapped = open.pop();
    }
}
