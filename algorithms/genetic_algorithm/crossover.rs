use rand::Rng;
use crate::core::Node;

pub fn crossover(parent1: &Vec<Node>, parent2: &Vec<Node>) -> Vec<Node> {
    let mut rng = rand::rng();
    let size = parent1.len();
    let mut child = vec![None; size];

    let start = rng.random_range(1..size / 2);
    let end = rng.random_range(size / 2..size-1);

    for i in start..end {
        child[i] = Some(parent1[i].clone());
    }

    let mut index = 0;
        for node in parent2 {
            if !child.contains(&Some(node.clone())) {
                while child[index].is_some() {
                    index += 1;
                }
                child[index] = Some(node.clone());
            }
        }

    child.into_iter().map(|x| x.unwrap()).collect()
}
