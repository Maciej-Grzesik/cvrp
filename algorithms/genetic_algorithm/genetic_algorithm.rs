use crate::core::{DistanceMatrix, Instance};
use crate::crossover::crossover;
use crate::evaluator::evaluate;
use crate::mutate::mutate;
use crate::tournament_select::tournament_selection;
use rand::seq::IndexedRandom;
use rand::{Rng, seq::SliceRandom};

pub fn genetic_algorithm(
    instance: &Instance,
    generations: i32,
    population_size: i32,
    crossover_probability: f64,
    mutation_probability: f64,
) -> f64 {
    let mut population: Vec<Vec<i32>> = vec![instance.nodes_id.clone(); population_size as usize];
    let distance_matrix: DistanceMatrix = DistanceMatrix::new(&instance.nodes);
    let mut rng = rand::rng();

    for individual in &mut population {
        individual.remove(0);
        individual.shuffle(&mut rng);
    }

    let mut best_run: f64 = f64::INFINITY;

    for _ in 0..(generations * population_size) {
        let mut offspring: Vec<Vec<i32>> = Vec::new();

        for _ in 0..(population_size) {
            let use_parent = rng.random_range(0.0..1.0) < 0.1;

            let mut parent1 =
                tournament_selection(&population, instance, &distance_matrix, &mut rng);
            let mut parent2 =
                tournament_selection(&population, instance, &distance_matrix, &mut rng);

            if !use_parent {
                parent1 = population.choose(&mut rng).unwrap().to_vec();
                parent2 = population.choose(&mut rng).unwrap().to_vec();
            }

            let mut child = parent1.clone();
            if rng.random_range(0.0..1.0) < crossover_probability {
                child = crossover(&parent1, &parent2, &mut rng);
            }
            if rng.random_range(0.0..1.0) < mutation_probability {
                mutate(&mut child, &mut rng);
            }
            offspring.push(child);
        }

        population = next_gen(&population, &offspring, instance, &distance_matrix);
        let current_run = evaluate(&distance_matrix, instance, &population[0]);
        if current_run < best_run {
            best_run = current_run;
        }
    }

    best_run
}

fn next_gen(
    population: &Vec<Vec<i32>>,
    offspring: &Vec<Vec<i32>>,
    instance: &Instance,
    distance_matrix: &DistanceMatrix,
) -> Vec<Vec<i32>> {
    let mut combined = population.clone();
    combined.extend(offspring.clone());

    combined.sort_by(|a, b| {
        let fitness_a = evaluate(distance_matrix, instance, a);
        let fitness_b = evaluate(distance_matrix, instance, b);

        fitness_a.partial_cmp(&fitness_b).unwrap()
    });

    combined.truncate(population.len());

    combined
}
