use rand::seq::SliceRandom;

use crate::core::{Instance, Node};
use crate::evaluator::evaluate;

pub fn greedy_search(instance: &Instance) -> (f64, Vec<Node>) {
    let generations: i32 = 100;

    let mut population: Vec<Node> = instance.nodes.clone();
    let slice = &mut population[1..];
    let mut rng = rand::rng();
    slice.shuffle(&mut rng);
    let (distance, path) = evaluate(instance, population);


    for i in 0..generations {
        
    }


    (distance, path)
}
