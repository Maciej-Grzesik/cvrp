use rand::seq::IndexedRandom;

use crate::core::{DistanceMatrix, Instance};
use crate::evaluator::evaluate;

pub fn tournament_selection(population: &Vec<Vec<i32>>, instance: &Instance, distance_matrix: &DistanceMatrix) -> Vec<i32> {
    let mut rng = rand::rng();
    let tournament_size = 2;
    let mut best = None;
    let mut best_fitness = f64::INFINITY;

    for _ in 0..tournament_size {
        let individial = population.choose(&mut rng).unwrap();
        let fitness = evaluate(distance_matrix, instance, individial);
        if fitness < best_fitness {
            best_fitness = fitness;
            best = Some(individial);
        }
    }

    best.unwrap().to_vec()
}
