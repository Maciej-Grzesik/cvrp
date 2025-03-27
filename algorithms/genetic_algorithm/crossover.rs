use rand::Rng;

pub fn crossover(parent1: &Vec<i32>, parent2: &Vec<i32>) -> Vec<i32> {
    let mut rng = rand::rng();
    let size = parent1.len();
    let mut child = vec![None; size];

    let start = rng.random_range(1..size / 2);
    let end = rng.random_range(size / 2..size - 1);

    for i in start..end {
        child[i] = Some(parent1[i]);
    }

    let mut index = 0;
    for node in parent2 {
        if !child.contains(&Some(*node)) {
            while child[index].is_some() {
                index += 1;
            }
            child[index] = Some(*node);
        }
    }

    child.into_iter().map(|x| x.unwrap()).collect()
}
