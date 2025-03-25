use std::collections::HashMap;

use rand::seq::SliceRandom;
use rand::Rng;

use crate::core::{Instance, Node};
use crate::evaluator::evaluate;


pub fn tabu_search(instance: &Instance) -> (f64, Vec<Node>) {
    let mut tabu_map: HashMap<TabuMoves, (i32, i32)> = HashMap::new();
    let mut path: Vec<Node> = instance.nodes.clone();
    let mut rng = rand::rng();
    path[1..].shuffle(&mut rng);
    let mut current_capacity: i32 = 0;

    let mut best_path = path.clone();

    for _ in 0..1000 {
        let mut a = rng.random_range(1..path.clone().len());
        let mut b = rng.random_range(a..=path.len());
        swap(&mut path, a, b);
        two_opt(&mut path, a, b);
        relocate(&mut path, a, b);
        let fitness = evaluate(instance, path.clone()).0;
        if fitness < evaluate(instance, best_path.clone()).0 {
            best_path = path.clone();
        }
    }

    (12.2, vec![Node {id: 1, x: 2, y: 3, demand: 10}])
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

enum TabuMoves {
    Swap,
    Relocate,
    TwoOpt,
}

