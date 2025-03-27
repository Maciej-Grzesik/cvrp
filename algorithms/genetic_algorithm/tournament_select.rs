use rand::seq::IndexedRandom;
use rand::Rng;

use crate::core::{DistanceMatrix, Instance};
use crate::evaluator::evaluate;
use rand::rngs::ThreadRng;

pub fn tournament_selection(population: &Vec<Vec<i32>>, instance: &Instance, distance_matrix: &DistanceMatrix, rng: &mut ThreadRng) -> Vec<i32> {
    let tournament_size = 5;
    let selection_pressure = 0.75;

    let mut tournament: Vec<&Vec<i32>> = population.choose_multiple(rng, tournament_size).collect();

    tournament.sort_by(|a, b| {
        let fitness_a = evaluate(distance_matrix, instance, a);
        let fitness_b = evaluate(distance_matrix, instance, b);

        fitness_a.partial_cmp(&fitness_b).unwrap()
    });

    let index = if rng.random_range(0.0..1.0) < selection_pressure {
        0
    } else {
        rng.random_range(1..tournament_size)
    };

    tournament[index].to_vec()
}
