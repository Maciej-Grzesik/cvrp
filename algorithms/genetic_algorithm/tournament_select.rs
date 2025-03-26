use rand::seq::IndexedRandom;

use crate::core::{Instance, Node};
use crate::evaluator::evaluate;

pub fn tournament_selection(population: &Vec<Vec<Node>>, instance: &Instance) -> Vec<Node> {
    let mut rng = rand::rng();
    let tournament_size = 5;
    let mut best = None;
    let mut best_fitness = f64::INFINITY;

    for _ in 0..tournament_size {
        let individial = population.choose(&mut rng).unwrap();
        let fitness = evaluate(instance, individial.clone());
        if fitness < best_fitness {
            best_fitness = fitness;
            best = Some(individial.clone());
        }
    }

    best.unwrap()
}
