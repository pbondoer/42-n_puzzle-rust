use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::SystemTime;

use types::Atom;
use types::Node;
use types::Problem;
use types::Puzzle;
use types::Solution;

use util::find_empty_pos;
use util::print_puzzle;
use util::xy;

const NEIGHBOR_DELTAS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn neighbors(puzzle: &Puzzle, pos: Atom, size: Atom) -> HashSet<(Puzzle, Atom)> {
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
        let cur_pos = y * size as i32 + x;

        cur.swap(pos as usize, cur_pos as usize);

        set.insert((cur, cur_pos as Atom));
    }

    assert!(!set.is_empty(), "set should not be empty");

    set
}

pub fn astar<'a>(problem: &'a Problem) -> Solution {
    let start_time = SystemTime::now();

    let mut open = BinaryHeap::new();
    let mut closed = HashSet::new();
    let mut from: HashMap<Puzzle, Puzzle> = HashMap::new();

    // Final path
    let mut path = Vec::new();

    // Add the first node
    let initial_h_result = (problem.heuristic)(&problem.start, &problem.end, problem.size);

    open.push(Node {
        array: problem.start.clone(),
        h_result: initial_h_result,
        g_result: 0,
        f_result: initial_h_result,
        pos: find_empty_pos(&problem.start),
    });

    // start poppin' nodes
    let mut node_wrapped = open.pop();
    let mut node;

    while node_wrapped != None {
        node = node_wrapped.unwrap();

        closed.insert(node.array.clone());

        if node.array == problem.end {
            // Done, time to unwind the path
            let mut current = node.array.clone();

            path.push(current.clone());
            while from.contains_key(&current) {
                current = from[&current].clone();
                path.push(current.clone());
            }
            break;
        }

        for raw_neighbor in neighbors(&node.array, node.pos, problem.size) {
            let (neighbor, neighbor_pos) = raw_neighbor;

            if closed.contains(&neighbor) {
                continue;
            }

            let g_result = node.g_result + 1;
            let h_result = (problem.heuristic)(&neighbor, &problem.end, problem.size);

            let f_result = (h_result * problem.h_weight) + (g_result * problem.g_weight);

            open.push(Node {
                array: neighbor.clone(),
                h_result,
                g_result,
                f_result,
                pos: neighbor_pos,
            });

            from.insert(neighbor.clone(), node.array.clone());
        }

        node_wrapped = open.pop();
    }

    path.reverse();

    // done
    Solution {
        problem,
        path,
        max_states: closed.len() + open.len(),
        opened_states: open.len() + closed.len(),
        current_open_states: open.len(),
        closed_states: closed.len(),
        time: start_time,
    }
}

pub fn print_solution(s: &Solution) {
    println!("-----------------");
    for p in &s.path {
        print_puzzle(&p, s.problem.size);
        println!("-----------------");
    }

    match &s.time.elapsed() {
        Ok(elapsed) => println!(" - Time elapsed: {:?}", elapsed),
        Err(_) => {}
    }
    println!(" - Solution length: {}", s.path.len() - 1);
    println!(" - Maximum states in memory: {}", s.max_states);
    println!(
        " - Open states at time of solution: {}",
        s.current_open_states
    );
    println!(" - Total opened states: {}", s.opened_states);
    println!(" - Total closed states: {}", s.closed_states);
    println!("-----------------");
}
