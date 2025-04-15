use rand::{Rng, rngs::ThreadRng};

pub fn crossover(parent1: &Vec<i32>, parent2: &Vec<i32>, rng: &mut ThreadRng) -> Vec<i32> {
    let size = parent1.len();
    let mut child = vec![0; size];

    let start = rng.random_range(1..size - 1);
    let end = rng.random_range(start + 1..size);

    child[start..=end].clone_from_slice(&parent1[start..=end]);

    let mut pos = (end + 1) % size;
    for &gene in parent2.iter() {
        if !child.contains(&gene) {
            child[pos] = gene;
            pos = (pos + 1) % size;
        }
    }

    child
}
