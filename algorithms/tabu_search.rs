use std::collections::VecDeque;

use rand::seq::{IndexedRandom, SliceRandom};
use rand::random_range;

use crate::core::{Instance, Node};
use crate::evaluator::evaluate;

pub fn tabu_search(instance: &Instance) -> (f64, Vec<Node>) {
    let mut tabu_map: VecDeque<TabuMoves> = VecDeque::new();
    let mut path: Vec<Node> = instance.nodes.clone();
    let mut rng = rand::rng();
    path[1..].shuffle(&mut rng);

    let mut best_fitness = evaluate(instance, path.clone()).0;
    let mut best_path = path.clone();

    for _ in 0..10000000 {
        let a = random_range(1..path.len());
        let b = random_range(a..path.len());

        let move_type = *[
            TabuMoves::Swap(a, b),
            TabuMoves::Relocate(a, b),
            TabuMoves::TwoOpt(a, b),
        ]
        .choose(&mut rng)
        .unwrap();

        if !tabu_map.contains(&move_type) || evaluate(instance, path.clone()).0 < best_fitness {
            match move_type {
                TabuMoves::Swap(i, j) => swap(&mut path, i, j),
                TabuMoves::Relocate(i, j) => relocate(&mut path, i, j),
                TabuMoves::TwoOpt(i, j) => relocate(&mut path, i, j),
            }

            let new_fitness = evaluate(instance, path.clone()).0;
            if new_fitness < best_fitness {
                best_path = path.clone();
                best_fitness = new_fitness;
            }

            tabu_map.push_back(move_type);
            if tabu_map.len() > 10 {
                tabu_map.pop_front();
            }
        }
    }

    (best_fitness, evaluate(instance, best_path).1)
}

fn swap(path: &mut Vec<Node>, a: usize, b: usize) {
    path.swap(a, b);
}

fn relocate(path: &mut Vec<Node>, a: usize, b: usize) {
    if a == b || a >= path.len() || b >= path.len() {
        return;
    }
    let element = path.remove(a);
    path.insert(b, element);
}

fn two_opt(path: &mut Vec<Node>, a: usize, b: usize) {
    if a >= b || b >= path.len() {
        return;
    }
    path[a..=b].reverse();
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
enum TabuMoves {
    Swap(usize, usize),
    Relocate(usize, usize),
    TwoOpt(usize, usize),
}
