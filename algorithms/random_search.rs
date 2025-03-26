use core::f64;

use rand::seq::SliceRandom;
use statrs::statistics::Statistics;

use crate::core::{Instance, Node};
use crate::evaluator::evaluate;

pub fn random_search(instance: &Instance, iterations: i32) -> (f64, f64, f64, f64) {
    let mut shortest_distance = f64::INFINITY;
    let mut longest_distance = -f64::INFINITY;
    let mut rng = rand::rng();

    let mut runs: Vec<f64> = Vec::new();

    for _ in 0..iterations {
        let mut path: Vec<Node> = instance.nodes.clone();
        let slice = &mut path[1..];
        slice.shuffle(&mut rng);
        let current_run = evaluate(instance, path.clone());
        if current_run < shortest_distance {
            shortest_distance = current_run;
        }
        if current_run > longest_distance {
            longest_distance = current_run;
        }

        runs.push(current_run);
    }

    let mean = runs.as_slice().mean();
    let std_dev = runs.std_dev();

    (shortest_distance, longest_distance, mean, std_dev)
}
