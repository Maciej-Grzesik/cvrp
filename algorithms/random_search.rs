use core::f64;
use rand::seq::SliceRandom;
use crate::core::{DistanceMatrix, Instance};
use crate::evaluator::evaluate;

pub fn random_search(instance: &Instance, iterations: i32) -> f64 {
    let mut shortest_distance = f64::INFINITY;
    let mut longest_distance = -f64::INFINITY;
    let mut rng = rand::rng();

    let mut runs: Vec<f64> = Vec::new();

    let distance_matrix: DistanceMatrix = DistanceMatrix::new(&instance.nodes);

    let mut path = instance.nodes_id.clone();

    for _ in 0..iterations {
        path[1..].shuffle(&mut rng);
        let current_run = evaluate(&distance_matrix, instance, &path);
        if current_run < shortest_distance {
            shortest_distance = current_run;
        }
        if current_run > longest_distance {
            longest_distance = current_run;
        }

        runs.push(current_run);
    }

    //let mean = runs.as_slice().mean();
    //let std_dev = runs.std_dev();

    shortest_distance
}
