use crate::core::{Instance, Node};
use rand::{seq::SliceRandom, Rng};
use crate::tournament_select::tournament_selection;
use crate::crossover::crossover;
use crate::mutate::mutate;
use crate::evaluator::evaluate;

pub fn genetic_algorithm(instance: &Instance) -> (f64, Vec<Node>) {
    let generations: i32 = 1000;
    let generation_size: usize = 1000;

    let mut population: Vec<Vec<Node>> = vec![instance.nodes.clone(); generation_size];
    let mut rng = rand::rng();

    for individual in &mut population {
       individual[1..].shuffle(&mut rng);
    }

    for _ in 0..generations {
        let mut offspring: Vec<Vec<Node>> = Vec::new();

        for _ in 0..(generation_size / 2) {
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

fn next_gen(population: &Vec<Vec<Node>>, offspring: &Vec<Vec<Node>>, instance: &Instance) -> Vec<Vec<Node>> {
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
