use crate::core::{Instance, Node};
use crate::crossover::crossover;
use crate::evaluator::evaluate;
use crate::mutate::mutate;
use crate::tournament_select::tournament_selection;
use rand::{Rng, seq::SliceRandom};

pub fn genetic_algorithm(
    instance: &Instance,
    generations: i32,
    population_size: i32,
) -> (f64, Vec<Node>) {
    let mut population: Vec<Vec<Node>> = vec![instance.nodes.clone(); population_size as usize];
    let mut rng = rand::rng();

    for individual in &mut population {
        individual[1..].shuffle(&mut rng);
    }

    for _ in 0..(generations * population_size) {
        let mut offspring: Vec<Vec<Node>> = Vec::new();

        for _ in 0..(population_size / 2) {
            let parent1 = tournament_selection(&population, instance);
            let parent2 = tournament_selection(&population, instance);

            let mut child = parent1.clone();
            if rng.random_range(0.0..1.0) < 0.7 {
                child = crossover(&parent1, &parent2);
            }

            if rng.random_range(0.0..1.0) < 0.05 {
                mutate(&mut child);
            }

            offspring.push(child);
        }

        population = next_gen(&population, &offspring, instance);
    }

    evaluate(instance, population[0].clone())
}

fn next_gen(
    population: &Vec<Vec<Node>>,
    offspring: &Vec<Vec<Node>>,
    instance: &Instance,
) -> Vec<Vec<Node>> {
    let mut combined = population.clone();
    combined.extend(offspring.clone());
    combined.sort_by(|a, b| {
        let fitness_a = evaluate(instance, a.to_vec()).0;
        let fitness_b = evaluate(instance, b.to_vec()).0;

        fitness_a.partial_cmp(&fitness_b).unwrap()
    });
    combined.truncate(population.len());
    combined
}
