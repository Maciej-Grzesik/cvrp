use core::f64;

use rand::seq::SliceRandom;

use crate::core::{Instance, Node};
use crate::evaluator::evaluate;

pub fn random_search(instance: &Instance) -> (f64, i32, Vec<Node>) {
    let mut shortest_distance = f64::INFINITY;
    let mut best_path: Vec<Node> = Vec::new(); 
    let mut best_index = 0;
    let mut rng = rand::rng();

    for i in 0..1e7 as i32 {
        let mut path: Vec<Node> = instance.nodes.clone();
        let slice = &mut path[1..];
        slice.shuffle(&mut rng);
        let (current_run, updated_path) = evaluate(instance, path.clone());
        if current_run < shortest_distance {
            best_index = i;
            shortest_distance = current_run;
            best_path = updated_path;
        }
    }

    (shortest_distance, best_index, best_path)
}
