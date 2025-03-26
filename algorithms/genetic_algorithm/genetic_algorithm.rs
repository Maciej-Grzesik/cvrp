use core::f64;

use crate::core::{DistanceMatrix, Instance, Node};
use crate::crossover::crossover;
use crate::evaluator::evaluate;
use crate::mutate::mutate;
use crate::tournament_select::tournament_selection;
use rand::{Rng, seq::SliceRandom};
use statrs::statistics::Statistics;

pub fn genetic_algorithm(
    instance: &Instance,
    generations: i32,
    population_size: i32,
) -> (f64, f64, f64, f64) {
    let mut population: Vec<Vec<Node>> = vec![instance.nodes.clone(); population_size as usize];
    let distance_matrix: DistanceMatrix = DistanceMatrix::new(&instance.nodes);
    let mut rng = rand::rng();

    for individual in &mut population { 
        individual[1..].shuffle(&mut rng);
    }

    let mut best_runs: Vec<f64> = Vec::new();
    let mut worst_runs: Vec<f64> = Vec::new();

    for _ in 0..(generations * population_size) {
        let mut offspring: Vec<Vec<Node>> = Vec::new();

        for _ in 0..(population_size / 2) {
            let parent1 = tournament_selection(&population, instance, &distance_matrix);
            let parent2 = tournament_selection(&population, instance, &distance_matrix);

            let mut child = parent1.clone();
            if rng.random_range(0.0..1.0) < 0.7 {
                child = crossover(&parent1, &parent2);
            }

            if rng.random_range(0.0..1.0) < 0.1 {
                mutate(&mut child);
            }

            offspring.push(child);
        }

        population = next_gen(&population, &offspring, instance, &distance_matrix);
        //best_runs.push(evaluate(instance, population[0].clone()));
        //worst_runs.push(evaluate(instance, population.last().unwrap().clone()));
    }
    
    let best_run_min = best_runs.iter().cloned().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(f64::INFINITY);
    let worst_run_max = worst_runs.iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(f64::NEG_INFINITY);
    let mean = best_runs.as_slice().mean();
    let std_dev = best_runs.std_dev();

    (best_run_min, worst_run_max, mean, std_dev)
}

fn next_gen(
    population: &Vec<Vec<Node>>,
    offspring: &Vec<Vec<Node>>,
    instance: &Instance,
    distance_matrix: &DistanceMatrix,
) -> Vec<Vec<Node>> {
    //let mut combined = population.clone();
    //combined.extend(offspring.clone());
    //combined.sort_by(|a, b| {
        //let fitness_a = evaluate(distance_matrix, instance);
        //let fitness_b = evaluate(distance_matrix, instance);

    //    fitness_a.partial_cmp(&fitness_b).unwrap()
    //});
    //combined.truncate(population.len());
    //combined
    vec![vec![Node {id: 1, x:1, y:1, demand:1 }]]
}
