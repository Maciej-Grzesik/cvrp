use std::collections::VecDeque;
use rand::seq::SliceRandom;
use statrs::statistics::Statistics;

use crate::core::{Instance, Node};
use crate::evaluator::evaluate;

pub fn tabu_search(instance: &Instance, iterations: i32, tabu_size: usize) -> (f64, f64, f64, f64, Vec<Node>) {
    let mut tabu_list: VecDeque<TabuMoves> = VecDeque::new();
    let mut path: Vec<Node> = instance.nodes.clone();
    let mut rng = rand::rng();
    path[1..].shuffle(&mut rng);

    let mut best_fitness = evaluate(instance, path.clone()).0;
    let mut best_path = path.clone();
    let mut worst_fitness = best_fitness;
    let mut runs: Vec<f64> = Vec::new();

    for _ in 0..iterations {
        let mut best_neighbor = None;
        let mut best_neighbor_fitness = f64::INFINITY;
        let mut best_move = None;

        for i in 1..path.len() {
            for j in i + 1..path.len() {
                let moves = [
                    TabuMoves::Swap(i, j),
                    TabuMoves::Relocate(i, j),
                    TabuMoves::TwoOpt(i, j),
                ];

                for &move_type in &moves {
                    let mut new_path = path.clone();

                    match move_type {
                        TabuMoves::Swap(a, b) => swap(&mut new_path, a, b),
                        TabuMoves::Relocate(a, b) => relocate(&mut new_path, a, b),
                        TabuMoves::TwoOpt(a, b) => two_opt(&mut new_path, a, b),
                    }

                    let new_fitness = evaluate(instance, new_path.clone()).0;

                    let is_tabu = tabu_list.contains(&move_type);
                    let aspiration_criteria = new_fitness < best_fitness;

                    if new_fitness < best_neighbor_fitness && (!is_tabu || aspiration_criteria) {
                        best_neighbor = Some(new_path.clone());
                        best_neighbor_fitness = new_fitness;
                        best_move = Some(move_type);
                    }
                }
            }
        }

        if let Some(new_path) = best_neighbor {
            path = new_path;
            if let Some(mv) = best_move {
                tabu_list.push_back(mv);
                if tabu_list.len() > tabu_size {
                    tabu_list.pop_front();
                }
            }

            if best_neighbor_fitness < best_fitness || rand::random::<f64>() < 0.05 {
                best_path = path.clone();
                best_fitness = best_neighbor_fitness;
            }

            if best_neighbor_fitness > worst_fitness {
                worst_fitness = best_neighbor_fitness;
            }

            runs.push(best_neighbor_fitness);
        }
    }

    let mean = runs.as_slice().mean();
    let std_dev = runs.std_dev();

    (best_fitness, worst_fitness, mean, std_dev, evaluate(instance, best_path).1)
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
